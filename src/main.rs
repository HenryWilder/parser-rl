use raylib::prelude::*;

pub mod plugin;
pub mod device;
pub mod cable;
pub mod rack;
pub mod palette;
pub mod hover;

use device::*;
use plugin::*;
use cable::*;
use rack::*;
use palette::*;
use hover::*;

pub struct Divider();

fn main() {
    let window_width = 1280.0;
    let window_height = 720.0;
    let (mut rl, thread) = raylib::init()
        .size(window_width as i32, window_height as i32)
        .title("Parser toy")
        .build();

    rl.set_target_fps(60);

    let mut divider_x = 200.0;
    let mut palette = Palette::new(Rectangle::new(0.0, 0.0, divider_x, window_height));
    let mut rack = Rack::new(Rectangle::new(0.0, divider_x, window_width - divider_x, window_height));

    let mut hover_handler = HoverHandler::new();
    let divider_h_extent = 5.0;

    while !rl.window_should_close() {
        // Update

        let dt = rl.get_frame_time();
        let mouse_pos = rl.get_mouse_position();

        if !hover_handler.is_dragging() {
            if (mouse_pos.x - divider_x).abs() <= divider_h_extent {
                hover_handler.start_hovering(HoverTarget::Divider);
            }
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(hovering) = hover_handler.hovering_mut() {
                hovering.is_dragging = hovering.target.is_draggable();
            }
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(dragging) = hover_handler.dragging_mut() {
                dragging.is_dragging = false;
            }
        }

        if let Some(dragging) = hover_handler.dragging() {
            match dragging.target {
                HoverTarget::Divider => {
                    divider_x = mouse_pos.x;
                    palette.rec.width = divider_x;
                    rack.rec.x = divider_x;
                    rack.rec.width = window_width as f32 - divider_x;
                },
                HoverTarget::PaletteDevice(_) => todo!(),
                HoverTarget::RackDevice(_) => todo!(),
            }
        }

        if let Some(hover) = hover_handler.hovering() {
            match hover {
                HoverTarget::Divider => rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_RESIZE_EW),
                _ => todo!(),
            };
        } else {
            rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        }

        // Drawing
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            
            rack.draw(&mut d);
            palette.draw(&mut d);

            d.draw_rectangle_rec(Rectangle::new(divider_x - 1.0, 0.0, 2.0, window_height), Color::GRAY);
        }
    }
}
