use raylib::prelude::*;

pub mod plugin;
pub mod device;
pub mod cable;
pub mod rack;
pub mod palette;

use device::*;
use plugin::*;
use cable::*;
use rack::*;
use palette::*;

fn main() {
    let window_width = 1280;
    let window_height = 720;
    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Parser toy")
        .build();

    rl.set_target_fps(60);

    let rack = Rack::new();
    let palette = Palette::new();
    let mut divider_x = 200.0;
    let mut is_dragging_divider = false;
    let divider_h_extent = 5;

    while !rl.window_should_close() {
        // Update

        let dt = rl.get_frame_time();
        let mouse_pos = rl.get_mouse_position();

        let is_hovering_divider = (mouse_pos.x - divider_x).abs() as i32 <= divider_h_extent;

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if is_hovering_divider {
                is_dragging_divider = true;
            }
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            is_dragging_divider = false;
        }

        if is_dragging_divider {
            divider_x = mouse_pos.x;
        }

        if is_hovering_divider || is_dragging_divider {
            rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_RESIZE_EW);
        } else {
            rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        }

        // Drawing
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            let divider_x = divider_x as i32;
            
            rack.draw(&mut d, divider_x, 0, window_width - divider_x, window_height);
            palette.draw(&mut d, 0, 0, divider_x, window_height);

            d.draw_rectangle(divider_x - 1, 0, 2, window_height, Color::GRAY);
        }
    }
}
