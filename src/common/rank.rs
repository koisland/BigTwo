use serde::{Deserialize, Serialize};
use std::cmp::Ordering::{Equal, Greater, Less};
use strum_macros::EnumIter;

/// Big 2 card ranks.
/// Two is the highest value.
#[derive(EnumIter, Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Rank {
    Two = 13,
    Ace = 12,
    King = 11,
    Queen = 10,
    Jack = 9,
    Ten = 8,
    Nine = 7,
    Eight = 6,
    Seven = 5,
    Six = 4,
    Five = 3,
    Four = 2,
    Three = 1,
}

impl Rank {
    /// Convert card rank to string.
    pub fn as_str(&self) -> String {
        let str_res = match *self {
            Rank::Two => "2",
            Rank::Ace => "A",
            Rank::King => "K",
            Rank::Queen => "Q",
            Rank::Jack => "J",
            Rank::Ten => "10",
            Rank::Nine => "9",
            Rank::Eight => "8",
            Rank::Seven => "7",
            Rank::Six => "6",
            Rank::Five => "5",
            Rank::Four => "4",
            Rank::Three => "3",
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
        (*self as usize).partial_cmp(&(*other as usize)).unwrap()
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
        Some((*self as usize).cmp(&(*other as usize)))
    }
}

mod tests {
    use super::Rank;

    #[test]
    fn test_rank_to_str() {
        let rank_ace = Rank::Ace;
        let rank_two = Rank::Two;

        assert_eq!(rank_ace.as_str(), "A");
        assert_eq!(rank_two.as_str(), "2");
    }

    #[test]
    fn test_rank_to_val() {
        let rank_ace = Rank::Ace;
        let rank_two = Rank::Two;

        assert_eq!(rank_ace as usize, 12);
        assert_eq!(rank_two as usize, 13);
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
