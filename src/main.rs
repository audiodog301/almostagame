use almostagame::audiostuff::{cpal_stuff, process_player_details, Saw};
use almostagame::gamestuff::PlayerDetails;
use almostagame::instructions::Instruction;

use macroquad::prelude::*;

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

    let mut player_details: PlayerDetails = PlayerDetails {
        pos: vec3(0.0, 0.0, 0.0),
        clicking: false,
    };

    let mut MOVE_SPEED: f32 = 0.1;

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

    let mut y_vel = 0.0;

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
        if is_key_down(KeyCode::LeftControl) {
            MOVE_SPEED = 0.25;
        } else {
            MOVE_SPEED = 0.1;
        }
        if is_key_pressed(KeyCode::Space) && position.y == 2.0 {
            y_vel = 0.4;
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

        position.y += y_vel;
        y_vel -= 0.035;
        if position.y < 2.0 {
            position.y = 2.0;
        }

        clear_background(Color::from_rgba(145, 229, 255, 255));

        // Going 3d!

        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });

        draw_plane(
            vec3(0.0, 0.0, 0.0),
            vec2(25.0, 25.0),
            None,
            Color::from_rgba(151, 255, 120, 255),
        );

        set_default_camera();
        draw_text("click to make a sound", 10.0, 40.0, 60.0, WHITE);

        player_details.clicking = is_mouse_button_down(MouseButton::Left);
        player_details.pos = position;

        let instructions = process_player_details(player_details);

        for instruction in instructions {
            sender.send(instruction);
        }

        next_frame().await
    }
}
