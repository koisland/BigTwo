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

    pub fn add(&mut self, hand: &[Card]) -> Result<&CardStack, &'static str> {
        let hand_res = Hand::new(hand);

        if let Ok(new_hand) = hand_res {
            // Check that added hand is the same as previous hand kind.
            if self.kind != HandType::None && self.kind != new_hand.kind {
                return Err("Current stack kind doesn't match previous stack kind.");
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

    pub fn clear(&mut self) -> &CardStack {
        self.stack.clear();
        self.kind = HandType::None;
        self
    }
}

mod tests {
    use super::*;
    use crate::logic::combo::{get_combos, get_dupes};

    #[test]
    fn test_create_stack() {
        let new_stack = CardStack::new();
    }

    #[test]
    fn test_add_single_to_stack() {
        let test_seq_file = "test/test_add_seq.json";
        let mut new_stack = CardStack::new();
        let cards: Vec<Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();

        for card in cards.into_iter() {
            if new_stack.add(&[card]).is_ok() {
                // println!("{:?}", new_stack)
            }
        }
    }

    #[test]
    fn test_add_double_to_stack() {
        let test_seq_file = "test/test_add_seq.json";
        let mut new_stack = CardStack::new();
        let cards: Vec<Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();

        if let Some(doubles) = get_dupes(&cards, 2) {
            for double in doubles.iter() {
                if let (Some(card_1), Some(card_2)) = (double.get(0), double.get(1)) {
                    if new_stack.add(&[*card_1, *card_2]).is_ok() {
                        // println!("{:?}", new_stack)
                    }
                }
            }
        }
    }

    #[test]
    fn test_add_combo() {
        let test_seq_file = "test/cards_dupes.json";
        let mut new_stack = CardStack::new();
        let mut cards: Vec<Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();

        let combos = get_combos(&cards);
        println!("{:#?}", combos)
    }
}
