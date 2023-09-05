use crate::prelude::*;
use raylib::prelude::*;

pub struct BounceSystem;
impl System for BounceSystem {
    fn run(&self, world: &World, _operation_stack: &mut OperationStack) {
        let query = world.query_components_mut::<(Position, Size, Dir)>();
        let rl = world.get_resource::<RaylibHandle>().unwrap();
        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();
        for (pos, size, mut dir) in query {
            let pos = pos.0;
            let size = size.0;
            if pos.x < 0.0 || pos.x > screen_width as f32 - size.x {
                dir.0.x *= -1.0;
            }
            if pos.y < 0.0 || pos.y > screen_height as f32 - size.y {
                dir.0.y *= -1.0;
            }
        }
    }
}

pub struct UpdateSystem;
impl System for UpdateSystem {
    fn run(&self, world: &World, _operation_stack: &mut OperationStack) {
        let query = world.query_components_mut::<(Position, Dir, Speed)>();
        let rl = world.get_resource::<RaylibHandle>().unwrap();
        for (mut position, dir, speed) in query {
            position.0 += dir.0 * speed.0 * rl.get_frame_time();
        }
    }
}

pub struct AddSquaresSystem;
impl System for AddSquaresSystem {
    fn run(&self, world: &World, operation_stack: &mut OperationStack) {
        let rl = world.get_resource::<RaylibHandle>().unwrap();
        let mut counter = world.get_resource_mut::<SquareCounter>().unwrap();
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let multi = 25usize;
            for i in 0..multi {
                add_square(world, operation_stack, i);
            }
            counter.0 += multi;
        }
    }
}

pub struct RenderSystem;
impl System for RenderSystem {
    fn run(&self, world: &World, _operation_stack: &mut OperationStack) {
        let thread = world.get_resource::<RaylibThread>().unwrap();
        let mut rl = world.get_resource_mut::<RaylibHandle>().unwrap();
        let counter = world.get_resource::<SquareCounter>().unwrap();
        let fps = rl.get_fps();
        let mut draw_handle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);
        let query = world.query_components::<(Renderable, Position, Size)>();
        for (renderable, position, size) in query {
            draw_handle.draw_rectangle_v(position.0, size.0, renderable.0);
        }
        draw_handle.draw_text(&fps.to_string(), 10, 0, 50, Color::LIME);
        draw_handle.draw_text(&counter.0.to_string(), 10, 50, 50, Color::LIME);
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

fn add_square(world: &World, operation_stack: &mut OperationStack, times_run: usize) {
    for _ in 0..(times_run + 1) {
        operation_stack.new_entity();
    }
    let square = world.next_slot() + times_run;
    let color = Color::new(
        get_random_value::<i32>(0, 255) as u8,
        get_random_value::<i32>(0, 255) as u8,
        get_random_value::<i32>(0, 255) as u8,
        255,
    );
    let dir = Vector2::new(
        if get_random_value::<i32>(0, 1) == 0 { -1.0 } else { 1.0 },
        if get_random_value::<i32>(0, 1) == 0 { -1.0 } else { 1.0 },
    )
    .normalized();
    let speed = get_random_value::<i32>(100, 800) as f32;
    let size = Vector2::new(
        get_random_value::<i32>(10, 100) as f32,
        get_random_value::<i32>(10, 100) as f32,
    );
    let pos = Vector2::new(
        get_random_value::<i32>(0, 800) as f32 - size.x,
        get_random_value::<i32>(0, 600) as f32 - size.y,
    );
    operation_stack.add_component(square, Renderable(color));
    operation_stack.add_component(square, Position(pos));
    operation_stack.add_component(square, Dir(dir));
    operation_stack.add_component(square, Speed(speed));
    operation_stack.add_component(square, Size(size));
}
