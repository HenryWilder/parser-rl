use raylib::prelude::*;

use crate::{Device, DeviceKind, Pane};

#[derive(Debug)]
pub struct Rack {
    pub pane: Pane,
    pub devices: Vec<Device>,
}

impl Rack {
    pub const DEVICE_GAP: f32 = 1.0;
    pub const RACK_WIDTH: f32 = 20.0;

    pub fn new(pane: Pane) -> Self {
        Self {
            pane,
            devices: Vec::new()
        }
    }

    pub fn insert_device(&mut self, index: usize, kind: DeviceKind) {
        let y = index.checked_sub(1)
            .and_then(|i| self.devices.get(i))
            .map_or(20.0, |device| {
                let rec = device.rectangle();
                rec.y + rec.height + Self::DEVICE_GAP
            });
        let device = Device::new(Vector2::new(Self::RACK_WIDTH, y), kind);
        for device in self.devices.iter_mut().skip(index) {
            device.move_y(device.rectangle().height + Self::DEVICE_GAP);
        }
        self.devices.insert(index, device);
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        for device in self.devices.iter() {
            device.draw(d);
        }
    }
}
