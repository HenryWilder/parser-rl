use raylib::prelude::*;

#[derive(Debug)]
pub struct Viewport {
    region: Rectangle,
    viewpoint: Vector2,
}

impl Viewport {
    pub fn new(region: Rectangle) -> Self {
        Self { region, viewpoint: Vector2::default() }
    }

    pub fn is_overlapping(&self, point: Vector2) -> bool {
        self.region.check_collision_point_rec(point)
    }

    pub fn begin_scissor<'d>(&self, d: &'d mut impl RaylibDraw) -> impl RaylibDraw + 'd {
        let Rectangle { x, y, width, height } = self.region;
        d.begin_scissor_mode(x as i32, y as i32, width as i32, height as i32)
    }

    #[allow(non_snake_case)]
    pub fn begin_2D<'d>(&self, d: &'d mut impl RaylibDraw) -> impl RaylibDraw + 'd {
        d.begin_mode2D(Camera2D {
            offset: Vector2::zero(),
            target: self.viewpoint,
            rotation: 0.0,
            zoom: 1.0,
        })
    }

    pub fn size(&self) -> Vector2 {
        Vector2::new(self.region.width, self.region.height)
    }
}
