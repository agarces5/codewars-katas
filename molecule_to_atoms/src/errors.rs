use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    Mismatch,
    Invalid,
    // variants
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Mismatch => write!(f, "Mismatched parenthesis"),
            ParseError::Invalid => write!(f, "Not a valid molecule"),
        }
    }
}
