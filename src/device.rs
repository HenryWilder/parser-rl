use raylib::prelude::*;

#[derive(Debug, Clone)]
pub enum Device {
    Label,
}

impl Device {
    pub fn measure(&self) -> Vector2 {
        match self {
            Self::Label => Vector2::new(100.0, 50.0),
        }
    }

    /// returns Vector2 { x: width, y: height }
    pub fn draw(&self, d: &mut impl RaylibDraw, pos: Vector2) -> Vector2 {
        let dim = self.measure();
        d.draw_rectangle_rec(Rectangle::new(pos.x, pos.y, dim.x, dim.y), Color::BLUE);
        dim
    }
}
