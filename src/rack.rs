use raylib::prelude::*;

use crate::{Device, DeviceKind, Viewport};

#[derive(Debug)]
pub struct Rack {
    pub pane: Viewport,
    pub devices: Vec<Device>,
}

impl Rack {
    pub const DEVICE_GAP: f32 = 1.0;
    pub const BEAM_WIDTH: f32 = 12.0;

    pub fn new(pane: Viewport) -> Self {
        Self {
            pane,
            devices: Vec::new()
        }
    }

    pub fn insert_device(&mut self, index: usize, kind: DeviceKind) {
        let y = index.checked_sub(1)
            .and_then(|i| self.devices.get(i))
            .map_or(Self::DEVICE_GAP, |device| {
                let rec = device.rectangle();
                rec.y + rec.height + Self::DEVICE_GAP
            });
        let device = Device::new(Vector2::new(Self::BEAM_WIDTH, y), kind);
        for device in self.devices.iter_mut().skip(index) {
            device.move_y(device.rectangle().height + Self::DEVICE_GAP);
        }
        self.devices.insert(index, device);
    }

    fn draw_beam(&self, x: f32, height: f32, d: &mut impl RaylibDraw) {
        d.draw_rectangle_rec(Rectangle::new(x, 0.0, Self::BEAM_WIDTH, height), Color::new(50, 40, 40, 255));
        d.draw_rectangle_rec(Rectangle::new(x + 2.5, 0.0, Self::BEAM_WIDTH - 4.0, height), Color::new(60, 47, 40, 255));
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        let mut d = self.pane.begin_scissor(d);
        let mut d = self.pane.begin_2D(&mut d);
        let Vector2 { x: width, y: height } = self.pane.size();
        d.clear_background(Color::BLACK);
        self.draw_beam(0.0, height, &mut d);
        self.draw_beam(Device::WIDTH + Self::BEAM_WIDTH, height, &mut d);
        for device in self.devices.iter() {
            device.draw(&mut d);
        }
    }
}
