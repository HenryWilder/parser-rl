use raylib::prelude::*;

pub enum Device {
    
}

impl Device {
    pub fn measure(&self) -> (i32, i32) {
        match self {
            _ => todo!(),
        }
    }

    /// returns (width, height)
    pub fn draw(&self, d: &mut impl RaylibDraw, x: i32, y: i32) -> (i32, i32) {
        let (width, height) = self.measure();
        d.draw_rectangle(x, y, width, height, Color::BLUE);
        (width, height)
    }
}
