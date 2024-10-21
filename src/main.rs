//! A parser in Raylib

#![warn(missing_docs)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_safety_doc)]

/// Reusable game components/traits
pub mod game_comp;
/// The gamer character
pub mod gamer;
/// Breaks strings into tokens
pub mod lexer;
/// Clumps tokens into syntax
pub mod parser;

use raylib::prelude::*;
use game_comp::*;
use gamer::Gamer;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Parser toy")
        .build();

    let mut gamer = Gamer::new(Transform2D::new(Vector2::zero(), 0.0));

    while !rl.window_should_close() {
        let movement = Vector2::new(
            (rl.is_key_down(KeyboardKey::KEY_D) as i32 - rl.is_key_down(KeyboardKey::KEY_A) as i32) as f32 * gamer.move_speed,
            (rl.is_key_down(KeyboardKey::KEY_S) as i32 - rl.is_key_down(KeyboardKey::KEY_W) as i32) as f32 * gamer.move_speed,
        );
        *gamer.position_mut() += movement;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_circle_v(gamer.position(), 10.0, Color::SKYBLUE);
    }
}
