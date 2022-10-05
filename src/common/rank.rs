use serde::{Deserialize, Serialize};
use std::cmp::Ordering::{Equal, Greater, Less};
use strum_macros::EnumIter;

/// Big 2 card ranks.
/// Two is the highest value.
#[derive(EnumIter, Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Rank {
    Two,
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
}

impl Rank {
    /// Convert card rank to value.
    pub fn as_value(&self) -> usize {
        match *self {
            Rank::Two => 13,
            Rank::Ace => 12,
            Rank::King => 11,
            Rank::Queen => 10,
            Rank::Jack => 9,
            Rank::Ten => 8,
            Rank::Nine => 7,
            Rank::Eight => 6,
            Rank::Seven => 5,
            Rank::Six => 4,
            Rank::Five => 3,
            Rank::Four => 2,
            Rank::Three => 1,
        }
    }
    /// Convert card rank to string.
    pub fn as_str(&self) -> String {
        let str_res = match *self {
            Rank::Two => "Two",
            Rank::Ace => "Ace",
            Rank::King => "King",
            Rank::Queen => "Queen",
            Rank::Jack => "Jack",
            Rank::Ten => "Ten",
            Rank::Nine => "Nine",
            Rank::Eight => "Eight",
            Rank::Seven => "Seven",
            Rank::Six => "Six",
            Rank::Five => "Five",
            Rank::Four => "Four",
            Rank::Three => "Three",
        };
        str_res.to_string()
    }
}

impl Ord for Rank {
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

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare rank based on value.
        self.as_value().partial_cmp(&other.as_value()).unwrap()
    }
}

impl PartialOrd for Rank {
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

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_value().cmp(&other.as_value()))
    }
}

mod tests {
    use super::Rank;

    #[test]
    fn test_rank_to_str() {
        let rank_ace = Rank::Ace;
        let rank_two = Rank::Two;

        assert_eq!(rank_ace.as_str(), "Ace");
        assert_eq!(rank_two.as_str(), "Two");
    }

    #[test]
    fn test_rank_to_val() {
        let rank_ace = Rank::Ace;
        let rank_two = Rank::Two;

        assert_eq!(rank_ace.as_value(), 12);
        assert_eq!(rank_two.as_value(), 13);
    }

    #[test]
    fn test_rank_cmp() {
        let rank_ace = Rank::Ace;
        let rank_two = Rank::Two;

        // Ranks equal.
        assert!(rank_ace.clone().eq(&rank_ace));

        // Ranks not equal.
        assert!(rank_two.ne(&rank_ace));

        // Ace is less than Two.
        assert!(rank_ace.lt(&rank_two));
    }
}
