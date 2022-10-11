use crate::common::card;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum StackType {
    Single,
    Double,
    Combo,
}

#[derive(Debug)]
pub struct CardStack {
    stack: Vec<Vec<card::Card>>,
    mode: Option<StackType>,
    level: usize,
}

impl CardStack {
    pub fn new() -> CardStack {
        CardStack {
            stack: Vec::new(),
            mode: None,
            level: 0,
        }
    }

    pub fn add(&mut self, hand: Vec<card::Card>) -> Result<&CardStack, &'static str> {
        let hand_type: StackType = match hand.len() {
            1 => StackType::Single,
            2 => StackType::Double,
            5 => StackType::Combo,
            _ => return Err("Invalid stack type."),
        };
        if let Some(current_hand_type) = &self.mode {
            if *current_hand_type != hand_type {
                return Err("Current stack type doesn' match previous stack type.");
            }
        }

        self.mode = Some(hand_type);

        // Check that addition to stack is valid.
        match self.mode {
            Some(StackType::Single) => {
                if let Some(last_hand) = self.stack.last() {
                    // If a card exists on stack...
                    // Otherwise, ignore.
                    if let Some(last_card) = last_hand.last() {
                        // Check current card.
                        let curr_card = &hand[0];
                        if curr_card.value() <= last_card.value() {
                            return Err("Current card is less than last card in stack.");
                        }
                    }
                }
            }
            Some(StackType::Double) => {
                if let Some(last_hand) = self.stack.last() {
                    // Calculate sum of value from doubles.
                    let curr_double_value: f32 = hand.iter().map(|card| card.value()).sum();
                    let last_double_value: f32 = last_hand.iter().map(|card| card.value()).sum();

                    if curr_double_value <= last_double_value {
                        return Err("Current double is less than last double.");
                    }
                }
            }
            Some(StackType::Combo) => todo!(),
            _ => todo!(),
        }

        self.level += 1;
        self.stack.push(hand);

        Ok(self)
    }

    pub fn clear(self) -> CardStack {
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
        let cards: Vec<card::Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();

        for card in cards.into_iter() {
            let hand = vec![card];

            // println!("Current hand {:?}", &hand);
            if new_stack.add(hand).is_ok() {
                // println!("{:?}", new_stack)
            }
        }
    }

    #[test]
    fn test_add_double_to_stack() {
        let test_seq_file = "test/test_add_seq.json";
        let mut new_stack = CardStack::new();
        let mut cards: Vec<card::Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();

        let doubles: Vec<Vec<card::Card>> = get_dupes(&cards, 2);
        for double in doubles.iter() {
            if let Some(card_1) = double.get(0) {
                if let Some(card_2) = double.get(1) {
                    // Get indices of doubles.
                    let double_idx: Vec<usize> = cards
                        .iter()
                        .enumerate()
                        .filter(|(_, card)| *card == card_1 || *card == card_2)
                        .map(|(idx, _)| idx)
                        .collect();

                    // Get doubles.
                    let double_hand = double_idx
                        .iter()
                        .map(|idx| cards.get(*idx).unwrap().clone())
                        .collect_vec();

                    // If addition to stack is valid.
                    if new_stack.add(double_hand).is_ok() {
                        // Remove added doubles from hand.
                        double_idx.iter().for_each(|idx| {
                            cards.remove(*idx);
                        })
                    }
                }
            }
        }

        println!("{:?}", new_stack);
    }
    #[test]
    fn test_add_combo() {
        let test_seq_file = "test/cards_dupes.json";
        let mut new_stack = CardStack::new();
        let mut cards: Vec<card::Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();

        let combos = get_combos(&cards);
        println!("{:#?}", combos)
    }
}
