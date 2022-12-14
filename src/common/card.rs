use crate::common::{rank::Rank, suit::Suit};

use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fmt::{self, Debug},
};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// Assign value of card based on rank and suit.
    pub fn value(&self) -> f32 {
        // rank (ace: 12) + ((spade: 4) / 10.0) -> 12.4
        (self.rank as usize) as f32 + ((self.suit as usize) as f32 / 10.0)
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}{}]", self.suit.as_str(), self.rank.as_str())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare cards based on value.
        self.value().partial_cmp(&other.value()).unwrap()
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        // Pattern `Some(Less | Eq)` optimizes worse than negating `None | Some(Greater)`.
        // FIXME: The root cause was fixed upstream in LLVM with:
        // https://github.com/llvm/llvm-project/commit/9bad7de9a3fb844f1ca2965f35d0c2a3d1e11775
        // Revert this workaround once support for LLVM 12 gets dropped.
        !matches!(self.partial_cmp(other), None | Some(Greater))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }
}

#[cfg(test)]
mod tests {
    use super::Card;
    use crate::common::{rank::Rank, suit::Suit};

    #[test]
    fn test_single_card_cmp() {
        let card_1 = Card {
            rank: Rank::Ace,
            suit: Suit::Club,
        };

        let card_2 = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };

        assert!(card_1.lt(&card_2))
    }
}
