use raylib::prelude::*;

/// A gamer character
#[derive(Debug)]
pub struct Gamer {
    /// The gamer's location in the world
    pub position: Vector2,
    /// How fast the gamer walks
    pub move_speed: f32,
}

impl Gamer {
    const DEFAULT_WALK_SPEED: f32 = 0.2;

    /// Construct a new gamer
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            move_speed: Self::DEFAULT_WALK_SPEED,
        }
    }
}
