use raylib::prelude::*;

pub struct Palette {

}

impl Palette {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, x: i32, y: i32, width: i32, height: i32) {
        let mut d = d.begin_scissor_mode(x, y, width, height);
        d.clear_background(Color::BLACK);
    }
}
