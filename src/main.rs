use almostagame::audiostuff::{cpal_stuff, Saw};
use almostagame::instructions::Instruction;

use macroquad::prelude::*;

#[macroquad::main("almostagame")]
async fn main() {
    let (sender, receiver) = crossbeam_channel::bounded(1024);
    cpal_stuff(receiver.clone());

    loop {
        clear_background(BLACK);

        draw_text("click to make a sound", 10.0, 40.0, 60.0, WHITE);

        if is_mouse_button_down(MouseButton::Left) {
            sender.send(Instruction::Set(1));
        } else {
            sender.send(Instruction::Set(0));
        }

        next_frame().await
    }
}
