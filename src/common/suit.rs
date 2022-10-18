use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

/// Standard card suit.
#[derive(EnumIter, Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Suit {
    Spade = 4,
    Heart = 3,
    Club = 2,
    Diamond = 1,
}

impl Suit {
    /// Convert card rank to string.
    pub fn as_str(&self) -> String {
        let str_res = match *self {
            Suit::Spade => "♠",
            Suit::Heart => "♥",
            Suit::Club => "♣",
            Suit::Diamond => "♦",
        };
        str_res.to_string()
    }
}
