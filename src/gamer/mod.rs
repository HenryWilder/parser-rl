use crate::game_comp::*;
use raylib::prelude::*;
use std::fmt;

/// A gamer character
pub struct Gamer {
    /// The gamer's location in the world
    pub transform: Transform2D,
    /// How fast the gamer walks
    pub move_speed: f32,
}

impl Positioned for Gamer {
    fn transform(&self) -> &Transform2D {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform2D {
        &mut self.transform
    }
}

impl Gamer {
    const DEFAULT_WALK_SPEED: f32 = 0.2;

    /// Construct a new gamer
    pub fn new(transform: Transform2D) -> Self {
        Self {
            transform,
            move_speed: Self::DEFAULT_WALK_SPEED,
        }
    }
}

impl fmt::Debug for Gamer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Gamer")
            .field("trans", &self.transform)
            .field("speed", &self.move_speed)
            .finish()
    }
}
