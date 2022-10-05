use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

/// Standard card suit.
#[derive(EnumIter, Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

impl Suit {
    /// Convert suit to value.
    /// Used in tandem with rank to determine card strength.
    pub fn as_value(&self) -> usize {
        match *self {
            Suit::Spade => 4,
            Suit::Heart => 3,
            Suit::Club => 2,
            Suit::Diamond => 1,
        }
    }
}
