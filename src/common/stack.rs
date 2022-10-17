use crate::common::{
    card::Card,
    hand::{Gauge, Hand, HandType},
};

use itertools::Itertools;

#[derive(Debug)]
pub struct CardStack {
    stack: Vec<Hand>,
    kind: HandType,
}

impl CardStack {
    pub fn new() -> CardStack {
        CardStack {
            stack: Vec::new(),
            kind: HandType::None,
        }
    }

    /// Adds a hand to the stack.
    ///
    /// Once a hand is added, the kind of hand is set and must be maintained until the stack is cleared.
    pub fn add(&mut self, hand: &[Card]) -> Result<&CardStack, &'static str> {
        let hand_res = Hand::new(hand);

        if let Ok(new_hand) = hand_res {
            // Check that added hand is the same as previous hand kind.
            if self.kind != HandType::None && self.kind != new_hand.kind {
                return Err("Current hand kind doesn't match previous stack kind.");
            }

            // Set the stack kind based on new hand added.
            self.kind = new_hand.kind.clone();

            // Check that hand beats previously based hand.
            if let Some(previous_hand) = self.stack.last() {
                match self.kind {
                    HandType::Single | HandType::Double | HandType::Combo => {
                        if new_hand.strength() < previous_hand.strength() {
                            return Err("Previous hand is stronger than added hand.");
                        }
                    }
                    _ => return Err("Invalid stack kind."),
                }
            }
            // Add hand to stack once validated.
            self.stack.push(new_hand);

            Ok(self)
        } else {
            Err(hand_res.unwrap_err())
        }
    }

    /// Clear the stack of cards.
    pub fn clear(&mut self) -> &CardStack {
        self.stack.clear();
        self.kind = HandType::None;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::hand::{
        tests::{get_test_hand, RelativeStrength},
        ComboType,
    };
    use crate::common::{rank::Rank, suit::Suit};
    use crate::logic::combo::get_dupes;

    #[test]
    fn test_create_stack() {
        let new_stack = CardStack::new();
        assert_eq!(new_stack.kind, HandType::None)
    }

    #[test]
    fn test_add_single_to_stack() {
        let test_seq_file = "test/test_add_seq.json";
        let mut new_stack = CardStack::new();
        let cards: Vec<Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();

        for card in cards.into_iter().sorted() {
            new_stack.add(&[card]).unwrap();
        }
    }

    #[test]
    fn test_add_double_to_stack() {
        let test_seq_file = "test/test_add_seq.json";
        let mut new_stack = CardStack::new();
        let cards: Vec<Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();

        if let Some(doubles) = get_dupes(&cards, 2) {
            for double in doubles.iter().sorted() {
                if let (Some(card_1), Some(card_2)) = (double.get(0), double.get(1)) {
                    new_stack.add(&[*card_1, *card_2]).unwrap();
                }
            }
        }
    }

    #[test]
    fn test_add_combo() {
        let mut new_stack = CardStack::new();
        let test_straight_res = get_test_hand(ComboType::Straight, RelativeStrength::Normal);
        let test_full_house_res = get_test_hand(ComboType::FullHouse, RelativeStrength::Normal);

        if let (Ok(straight), Ok(full_house)) = (test_straight_res, test_full_house_res) {
            new_stack.add(&straight.cards).unwrap();
            new_stack.add(&full_house.cards).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn test_add_invalid_weaker_combo() {
        let mut new_stack = CardStack::new();
        let test_bomb_res = get_test_hand(ComboType::Bomb, RelativeStrength::Normal);
        let test_full_house_res = get_test_hand(ComboType::FullHouse, RelativeStrength::Normal);

        if let (Ok(bomb), Ok(full_house)) = (test_bomb_res, test_full_house_res) {
            // Bombs are stronger than full houses.
            new_stack.add(&bomb.cards).unwrap();
            new_stack.add(&full_house.cards).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn test_add_invalid_different_kind_combo() {
        let mut new_stack = CardStack::new();
        let test_single = vec![Card {
            rank: Rank::Ace,
            suit: Suit::Club,
        }];
        let test_straight_res = get_test_hand(ComboType::Straight, RelativeStrength::Normal);

        if let Ok(straight) = test_straight_res {
            // Cannot add single after a combo has been set.
            new_stack.add(&straight.cards).unwrap();
            new_stack.add(&test_single).unwrap();
        }
    }

    #[test]
    fn test_clear_stack() {
        let mut new_stack = CardStack::new();
        let test_single = vec![Card {
            rank: Rank::Ace,
            suit: Suit::Club,
        }];
        let test_straight_res = get_test_hand(ComboType::Straight, RelativeStrength::Normal);

        if let Ok(straight) = test_straight_res {
            // Add a combo.
            new_stack.add(&straight.cards).unwrap();

            // Clear the stack and allowing another kind of hand.
            new_stack.clear();

            new_stack.add(&test_single).unwrap();
        }
    }
}
