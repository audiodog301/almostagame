use almostagame::audiostuff::{cpal_stuff, Saw};
use almostagame::instructions::Instruction;

use macroquad::prelude::*;

#[macroquad::main("almostagame")]
async fn main() {
    let (sender, receiver) = crossbeam_channel::bounded(1024);
    cpal_stuff(receiver.clone());

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        if is_mouse_button_down(MouseButton::Left) {
            sender.send(Instruction::Set(1));
        } else {
            sender.send(Instruction::Set(0));
        }

        next_frame().await
    }
}
