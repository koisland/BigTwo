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
