use std::cmp::Ordering;

use crate::common::{card, rank, suit};
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq)]
pub enum StackType {
    Single,
    Double,
    Combo,
}

pub fn get_dupes(hand: &[card::Card], size: usize) -> Vec<Vec<card::Card>> {
    // Sort hand before finding duplicates.
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();

    let duplicates: Vec<Vec<card::Card>> = hand_copy
        .iter()
        .enumerate()
        .filter_map(|(i, card)| {
            let cards_slice = hand_copy.as_slice();
            let mut next_cards = cards_slice[i..].iter();
            // Add the card being iterated over.
            let mut dupes: Vec<card::Card> = vec![];
            
            // Then start from self and look n cards ahead.
            for _ in 0..size {
                if let Some(next_card) = next_cards.next() {
                    if card.rank == next_card.rank {
                        dupes.push(next_card.clone())
                    }
                }
            }
            // If duplicates found doesn't equal the length of the desired duplicates.
            if dupes.len() != size {
                None
            } else {
                Some(dupes)
            }

        })
        .collect();
    duplicates
}

pub fn get_bombs(hand: &[card::Card]) {

}
pub fn get_full_house(hand: &[card::Card]) {
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();
}

pub fn get_straights(hand: &[card::Card]) {
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();
    // straight
    // straight flush
    // royal flush
    let mut contig_cards: Vec<Vec<card::Card>> = vec![];
    let mut interm_contig_cards: Vec<card::Card> = vec![];
    for card in hand_copy.iter() {
        // Check last card in intermediate straight.
        if let Some(last_card) = interm_contig_cards.last() {
            
            let card_rank_value = card.rank.as_value();
            let last_card_rank_value = last_card.rank.as_value();

            let rank_diff = card_rank_value as i8 - last_card_rank_value as i8;

            if rank_diff == 1 || rank_diff == 0 {
                interm_contig_cards.push(card.clone());
            } else {
                // Ignore intermediates less than 5.
                if interm_contig_cards.len() > 5 {
                    contig_cards.push(interm_contig_cards.clone());
                }
                interm_contig_cards.clear()
            }
        } else {
            interm_contig_cards.push(card.clone())
        }
    }

    contig_cards.push(interm_contig_cards.clone());

    let straights = contig_cards
        .iter()
        .map(|seq_cards| 
            seq_cards
                .iter()
                .permutations(5)
                .filter(|card_perm|{
                    let perm = card_perm
                        .iter()
                        .map(|card| card.rank)
                        .collect_vec();

                    let all_uniq =  perm.iter().all_unique();
                    let is_sorted = perm.into_iter().tuple_windows().all(|(a,b)| a < b);

                    all_uniq && is_sorted
                    })
                .collect_vec())
        .collect_vec();

    println!("Possible values: {:#?}", straights);
}

pub fn get_flushes(hand: &[card::Card]) {
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();
    // flushes
    let mut possible_flush: Vec<Vec<card::Card>> = vec![];
    for suit in suit::Suit::iter() {
        let suit_cards = hand_copy
            .iter()
            .filter_map(|card| if card.suit == suit { Some(card.clone()) } else { None })
            .collect_vec();
        if suit_cards.len() >= 5 {
            possible_flush.push(suit_cards);
        }
    }
    println!("Possible flushes: {:#?}", possible_flush);
}
pub fn get_combos(hand: &Vec<card::Card>) {
    // recursive solution?
    // find cards, pop from vec, and call itself?
    let mut hand_copy = hand.clone();
    hand_copy.sort();

    // full house
    // bomb (4 of kind + 1)
}

#[derive(Debug)]
pub struct CardStack {
    stack: Vec<Option<Vec<card::Card>>>,
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
        let hand_type = match hand.len() {
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
                    if let Some(last_card) = &last_hand {
                        // Check current card.
                        let curr_card = &hand[0];
                        if curr_card.value() <= last_card[0].value() {
                            return Err("Current card is less than last card in stack.");
                        }
                    }
                }
            }
            Some(StackType::Double) => {
                if let Some(last_hand) = self.stack.last() {
                    if let Some(last_double) = &last_hand {
                        // Calculate sum of value from doubles.
                        let curr_double_value: f32 = hand.iter().map(|card| card.value()).sum();
                        let last_double_value: f32 =
                            last_double.iter().map(|card| card.value()).sum();

                        if curr_double_value <= last_double_value {
                            return Err("Current double is less than last double.");
                        }
                    }
                }
            }
            Some(StackType::Combo) => todo!(),
            _ => todo!(),
        }

        self.level += 1;
        let final_hand: Vec<card::Card> = hand.into_iter().map(|card| card.clone()).collect();
        self.stack.push(Some(final_hand));

        Ok(self)
    }

    pub fn clear(self) -> CardStack {
        self
    }
}

mod tests {
    use super::*;
    use itertools::Itertools;
    use serde_json::from_reader;
    use std::fs::File;

    #[test]
    fn test_create_stack() {
        let new_stack = CardStack::new();
    }

    #[test]
    fn test_add_single_to_stack() {
        let test_seq_file = "test/test_add_seq.json";
        let mut new_stack = CardStack::new();
        let cards: Vec<card::Card> =
            serde_json::from_reader(&File::open(test_seq_file).unwrap()).unwrap();

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
            serde_json::from_reader(&File::open(test_seq_file).unwrap()).unwrap();

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
            serde_json::from_reader(&File::open(test_seq_file).unwrap()).unwrap();

        let triples = get_dupes(&cards, 2);
        println!("{:#?}", triples)
    }
}
