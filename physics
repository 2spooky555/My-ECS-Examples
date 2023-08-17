#![recursion_limit = "512"]
pub mod components;
pub mod game_systems;
pub mod operations;
pub mod resources;
pub mod systems;
pub mod world;
use game_systems::*;
use operations::*;
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
