use crate::game_comp::*;
use std::fmt;

/// Tokens recognized by the parser
pub enum Token {
    /// Addition
    Plus,
    /// Subtraction or negation
    Minus,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Plus => write!(f, "[+]"),
            Token::Minus => write!(f, "[-]"),
        }
    }
}

/// A box representing a token
pub struct Block {
    /// The transform component of the block
    pub transform: Transform2D,
    /// The token this block represents
    pub token: Token,
}

impl Positioned for Block {
    fn transform(&self) -> &Transform2D {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform2D {
        &mut self.transform
    }
}
