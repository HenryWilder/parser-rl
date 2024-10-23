use raylib::prelude::*;

use crate::Device;

#[derive(Debug)]
pub struct Palette {
    pub rec: Rectangle,
    /// `list` rectangles are relative to `rec`
    pub list: Vec<(Rectangle, Device)>,
}

impl Palette {
    const INSET: f32 = 5.0;
    const GAP: f32 = 7.0;
    const DEVICES: [Device; 1] = [
        Device::Label,
    ];

    pub fn new(rec: Rectangle) -> Self {
        let x = rec.x + Self::INSET;
        let mut y = rec.y + Self::INSET;
        Self {
            rec,
            list: Self::DEVICES.iter()
                .map(|device| {
                    let dim = device.measure();
                    let item = (Rectangle::new(x, y, dim.x, dim.y), device.clone());
                    y += dim.y + Self::GAP;
                    item
                })
                .collect(),
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        let Rectangle { x, y, width, height } = self.rec;
        let mut d = d.begin_scissor_mode(x as i32, y as i32, width as i32, height as i32);
        d.clear_background(Color::BLACK);
        for (rec, device) in self.list.iter() {
            device.draw(&mut d, Vector2::new(rec.x, rec.y));
        }
    }

    pub fn device_overlapping_point(&self, point: Vector2) -> Option<(Rectangle, &Device)> {
        if self.rec.check_collision_point_rec(point) {
            for (rec, device) in self.list.iter() {
                if rec.check_collision_point_rec(point) {
                    return Some((*rec, device));
                }
            }
        }
        None
    }
}
