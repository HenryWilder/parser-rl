use raylib::prelude::*;

use crate::Device;

#[derive(Debug)]
pub struct Rack {
    pub rec: Rectangle,
    pub devices: Vec<Device>,
}

impl Rack {
    pub fn new(rec: Rectangle) -> Self {
        Self {
            rec,
            devices: Vec::new()
        }
    }

    fn device_rec_iter(&self) -> impl Iterator<Item = (Rectangle, &Device)> {
        let x = self.rec.x;
        let mut y = self.rec.y;
        self.devices.iter()
            .map(move |device| {
                let dim = device.measure();
                let item = (Rectangle::new(x, y, dim.x, dim.y), device);
                y += dim.y;
                item
            })
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        let Rectangle { x, y, width, height } = self.rec;
        let mut d = d.begin_scissor_mode(x as i32, y as i32, width as i32, height as i32);
        d.clear_background(Color::BLACK);
        for (rec, device) in self.device_rec_iter() {
            device.draw(&mut d, Vector2::new(rec.x, rec.y));
        }
    }
}
