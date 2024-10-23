use raylib::prelude::*;

use crate::Device;

pub struct Rack {
    pub devices: Vec<Device>,
}

impl Rack {
    pub fn new() -> Self {
        Self {
            devices: Vec::new()
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, x: i32, y: i32, width: i32, height: i32) {
        let mut d = d.begin_scissor_mode(x, y, width, height);
        d.clear_background(Color::BLACK);
        {
            let mut y1 = y;
            for device in self.devices.iter() {
                let (_w, h) = device.draw(&mut d, x, y1);
                y1 += h;
            }
        }
    }
}
