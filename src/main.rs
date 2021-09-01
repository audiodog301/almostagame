use almostagame::audiostuff::{cpal_stuff, Saw};
use almostagame::instructions::Instruction;

use macroquad::prelude::*;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1920,
        window_height: 1080,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let (sender, receiver) = crossbeam_channel::bounded(1024);
    cpal_stuff(receiver.clone());

    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up;

    let mut position = vec3(0.0, 2.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::W) {
            position += vec3(front.x, 0.0, front.z) * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            position -= vec3(front.x, 0.0, front.z) * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;
        pitch += mouse_delta.y * delta * -LOOK_SPEED;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        clear_background(LIGHTGRAY);

        // Going 3d!

        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        set_default_camera();
        draw_text("click to make a sound", 10.0, 40.0, 60.0, WHITE);

        if is_mouse_button_down(MouseButton::Left) {
            sender.send(Instruction::Set(1));
        } else {
            sender.send(Instruction::Set(0));
        }

        next_frame().await
    }
}
