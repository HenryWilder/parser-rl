use raylib::prelude::*;

#[derive(Debug)]
pub struct Pane {
    region: Rectangle,
}

impl Pane {
    pub fn new(region: Rectangle) -> Self {
        Self { region }
    }

    pub fn is_overlapping(&self, point: Vector2) -> bool {
        self.region.check_collision_point_rec(point)
    }
}
