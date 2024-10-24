use raylib::prelude::*;

pub mod plugin;
pub mod device;
pub mod cable;
pub mod rack;
pub mod palette;
pub mod viewport;

use device::*;
use plugin::*;
use cable::*;
use rack::*;
use palette::*;
use viewport::*;

// there is no reason this needs a raylib handle
pub fn measure_text(text: &str, font_size: i32) -> i32 {
    let c_text = std::ffi::CString::new(text).unwrap();
    unsafe { ffi::MeasureText(c_text.as_ptr(), font_size) }
}

fn main() {
    let window_width = 1280.0;
    let window_height = 720.0;
    let (mut rl, thread) = raylib::init()
        .size(window_width as i32, window_height as i32)
        .title("Parser toy")
        .msaa_4x()
        .build();

    rl.set_target_fps(60);

    // let mut palette = Palette::new(Pane::new(Rectangle::new(0.0, 0.0, 400.0, 1000.0)));
    let mut rack = Rack::new(Viewport::new(Rectangle::new(0.0, 0.0, window_width, window_height)));

    let device_0 = rack.insert_device(0, DeviceKind::Label(String::from("Test")));
    let device_1 = rack.insert_device(1, DeviceKind::Immediate(Value::I32(-363)));
    let device_2 = rack.insert_device(2, DeviceKind::Immediate(Value::U32(654)));
    let device_3 = rack.insert_device(3, DeviceKind::Immediate(Value::F32(5.63)));
    let device_4 = rack.insert_device(4, DeviceKind::Math(Operation::Add));
    // Cable::new();

    while !rl.window_should_close() {
        // Update

        let dt = rl.get_frame_time();
        let mouse_pos = rl.get_mouse_position();

        // Drawing
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            rack.draw(&mut d);
            // palette.draw(&mut d);
        }
    }
}
