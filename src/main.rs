//! A parser in Raylib

#![warn(missing_docs)]

/// The gamer character
pub mod gamer;
/// Breaks strings into tokens
pub mod lexer;
/// Clumps tokens into syntax
pub mod parser;

use raylib::prelude::*;
use gamer::Gamer;

fn main() {
    let window_width = 1280;
    let window_height = 720;
    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Parser toy")
        .build();

    rl.set_target_fps(60);

    let mut gamer = Gamer::new(Vector2 { x: 100.0, y: 100.0 });
    let cam = Camera2D {
        offset: Vector2::zero(),
        target: Vector2::zero(),
        rotation: 0.0,
        zoom: 0.35,
    };

    while !rl.window_should_close() {
        // Update

        let dt = rl.get_frame_time();

        let movement = Vector2::new(
            (rl.is_key_down(KeyboardKey::KEY_D) as i32 - rl.is_key_down(KeyboardKey::KEY_A) as i32) as f32 * gamer.move_speed * dt,
            (rl.is_key_down(KeyboardKey::KEY_S) as i32 - rl.is_key_down(KeyboardKey::KEY_W) as i32) as f32 * gamer.move_speed * dt,
        );
        gamer.position += movement;

        // Drawing
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            // Camera
            {
                let mut d = d.begin_mode2D(cam);
        
                d.draw_circle_v(gamer.position, 50.0, Color::BLUE);
            }
        }
    }
}
