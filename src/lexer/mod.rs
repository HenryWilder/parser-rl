use raylib::prelude::*;
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
    pub transform: Vector2,
    /// The token this block represents
    pub token: Token,
}
