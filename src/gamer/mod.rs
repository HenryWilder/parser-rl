use raylib::prelude::*;
use std::fmt;

/// A gamer character
pub struct Gamer {
    /// The gamer's location in the world
    pub position: Vector2,
    /// How fast the gamer walks
    pub move_speed: f32,
}

impl Gamer {
    /// In cm/s
    const DEFAULT_WALK_SPEED: f32 = 900.0;

    /// Construct a new gamer
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            move_speed: Self::DEFAULT_WALK_SPEED,
        }
    }
}

impl fmt::Debug for Gamer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { position: Vector2 { x, y }, move_speed } = self;
        f.debug_struct("Gamer")
            .field("pos", &format!("(x: {x}cm, y: {y}cm)"))
            .field("speed", &format!("{move_speed}cm/s"))
            .finish()
    }
}
