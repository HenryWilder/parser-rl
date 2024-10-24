use raylib::prelude::*;
use crate::{Device, Pane};

#[derive(Debug)]
pub struct Palette {
    pub pane: Pane,
    /// `list` rectangles are relative to `rec`
    pub list: Vec<Device>,
}

impl Palette {
    const INSET: f32 = 5.0;
    const GAP: f32 = 7.0;

    const ALL_DEVICES: [Device; 0] = [

    ];

    pub fn new(pane: Pane) -> Self {
        Self {
            pane,
            list: Self::ALL_DEVICES.to_vec(),
        }
    }
}
