#![recursion_limit = "512"]
pub mod components;
pub mod game_systems;
pub mod operations;
pub mod resources;
pub mod systems;
pub mod world;
use operations::*;
use systems::System;
use rapier2d::prelude::*;
use raylib::prelude::*;
use world::World;

struct Gravity(Vector2);
struct Ground;
struct Ball;

register_components!(Ball, Ground, ColliderHandle, Texture2D);

register_resources!(
    RaylibHandle,
    RaylibThread,
    Gravity,
    IntegrationParameters,
    PhysicsPipeline,
    IslandManager,
    BroadPhase,
    NarrowPhase,
    ImpulseJointSet,
    MultibodyJointSet,
    CCDSolver,
    RigidBodySet,
    ColliderSet
);

const FPS: u32 = 60;

fn main() {
    /*
       TODO:
       Get an entity id from components
    */
    set_trace_log(TraceLogLevel::LOG_NONE);
    let (mut rl, thread) = raylib::init().size(800, 600).title("Hello World").fullscreen().build();
    rl.set_target_fps(FPS);

    let mut world = World::new();
    let mut operations = OperationStack::new();

    // physics
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // ground
    let ground_entity = world.new_entity();
    let collider = ColliderBuilder::cuboid(50.0, 50.0)
        .translation(vector![400.0, 400.0])
        .rotation(30.0f32.to_radians())
        .build();
    let ground_collider_handle = collider_set.insert(collider);
    world.add_component(ground_entity, ground_collider_handle);
    world.add_component(ground_entity, Ground);

    // bouncing ball
    let ball_entity = world.new_entity();
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![400.0, 0.0])
        .build();
    let collider = ColliderBuilder::ball(50.0).restitution(0.7).build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    let ball_collider_handle =
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);
    world.add_component(ball_entity, ball_collider_handle);
    world.add_component(ball_entity, Ball);

    let gravity = Gravity(Vector2::new(0.0, 100.0));
    let integration_parameters = IntegrationParameters {
        dt: 1.0 / FPS as f32,
        ..Default::default()
    };
    let physics_pipeline = PhysicsPipeline::default();
    let island_manager = IslandManager::default();
    let broad_phase = BroadPhase::default();
    let narrow_phase = NarrowPhase::default();
    let impulse_joint_set = ImpulseJointSet::new();
    let multibody_joint_set = MultibodyJointSet::new();
    let ccd_solver = CCDSolver::new();

    world.add_resource(gravity);
    world.add_resource(integration_parameters);
    world.add_resource(physics_pipeline);
    world.add_resource(island_manager);
    world.add_resource(broad_phase);
    world.add_resource(narrow_phase);
    world.add_resource(impulse_joint_set);
    world.add_resource(multibody_joint_set);
    world.add_resource(ccd_solver);
    world.add_resource(rigid_body_set);
    world.add_resource(collider_set);

    world.add_resource(rl);
    world.add_resource(thread);

    world.add_system(PhysicsSystem);
    world.add_system(RenderSystem);
    world.add_system(CloseSystem);

    world.run(&mut operations);
}

pub struct RenderSystem;
impl System for RenderSystem {
    fn run(&self, world: &World, _operation_stack: &mut OperationStack) {
        let thread = world.get_resource::<RaylibThread>().unwrap();
        let mut rl = world.get_resource_mut::<RaylibHandle>().unwrap();
        // get colliders handles
        let collider_set = world.get_resource::<ColliderSet>().unwrap();
        let ball_query = world.query_components::<(Ball, ColliderHandle)>();
        let (_, ball_collider_handle) = ball_query.get(0).unwrap();
        let ground_query = world.query_components::<(Ground, ColliderHandle)>();
        let (_, ground_collider_handle) = ground_query.get(0).unwrap();

        let ball_collider = collider_set.get(**ball_collider_handle).unwrap();
        let ground_collider = collider_set.get(**ground_collider_handle).unwrap();

        let mut draw_handle = rl.begin_drawing(&thread);

        draw_handle.clear_background(Color::WHITE);
        draw_cuboid_collider(&mut draw_handle, ground_collider, Color::RED);
        draw_circle_collider(&mut draw_handle, ball_collider, Color::BLUE);
        draw_handle.draw_fps(0, 0);
    }
}

pub struct CloseSystem;
impl System for CloseSystem {
    fn run(&self, world: &World, operation_stack: &mut OperationStack) {
        let rl = world.get_resource::<RaylibHandle>().unwrap();
        if rl.window_should_close() {
            operation_stack.stop_run();
        }
    }
}

pub struct PhysicsSystem;
impl System for PhysicsSystem {
    fn run(&self, world: &World, _operation_stack: &mut OperationStack) {
        let gravity = world.get_resource::<Gravity>().unwrap();
        let integration_parameters = world.get_resource::<IntegrationParameters>().unwrap();
        let mut physics_pipeline = world.get_resource_mut::<PhysicsPipeline>().unwrap();
        let mut island_manager = world.get_resource_mut::<IslandManager>().unwrap();
        let mut broad_phase = world.get_resource_mut::<BroadPhase>().unwrap();
        let mut narrow_phase = world.get_resource_mut::<NarrowPhase>().unwrap();
        let mut rigid_body_set = world.get_resource_mut::<RigidBodySet>().unwrap();
        let mut collider_set = world.get_resource_mut::<ColliderSet>().unwrap();
        let mut impulse_joint_set = world.get_resource_mut::<ImpulseJointSet>().unwrap();
        let mut multibody_joint_set = world.get_resource_mut::<MultibodyJointSet>().unwrap();
        let mut ccd_solver = world.get_resource_mut::<CCDSolver>().unwrap();

        physics_pipeline.step(
            &vector![gravity.0.x, gravity.0.y],
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            None,
            &(),
            &(),
        );
    }
}

fn draw_circle_collider(draw_handle: &mut RaylibDrawHandle, collider: &Collider, color: Color) {
    let translation = collider.translation();
    let center = Vector2::new(translation.x, translation.y);
    let radius = collider
        .shape()
        .as_ball()
        .unwrap_or_else(|| panic!("Collider must be a ball!"))
        .radius;
    draw_handle.draw_circle_v(center, radius, color);
}

fn draw_cuboid_collider(draw_handle: &mut RaylibDrawHandle, collider: &Collider, color: Color) {
    let shape = collider
        .shape()
        .as_cuboid()
        .unwrap_or_else(|| panic!("Collider must be a cuboid!"))
        .half_extents
        * 2.0;
    let size = Vector2::new(shape.x, shape.y);
    let center = collider.translation();
    let rotation = collider.rotation().angle().to_degrees();
    let rect = Rectangle::new(center.x, center.y, size.x, size.y);
    let rotation_point = Vector2::new(size.x / 2.0, size.y / 2.0);
    draw_handle.draw_rectangle_pro(rect, rotation_point, rotation, color);
}
