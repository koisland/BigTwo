use std::ops::Index;

use crate::common::{card, suit};
use itertools::Itertools;
use strum::IntoEnumIterator;

type Hand<'a> = (
    &'a card::Card,
    &'a card::Card,
    &'a card::Card,
    &'a card::Card,
    &'a card::Card,
);

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
                        dupes.push(*next_card)
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
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();
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
    let mut duplicate_cards: Vec<card::Card> = vec![];
    let mut interm_contig_cards: Vec<card::Card> = vec![];
    let mut interm_duplicates: Vec<card::Card> = vec![];
    for card in hand_copy.iter() {
        // Check last card in intermediate straight.
        if let Some(last_card) = interm_contig_cards.last() {
            let card_rank_value = card.rank.as_value();
            let last_card_rank_value = last_card.rank.as_value();

            let rank_diff = card_rank_value as i8 - last_card_rank_value as i8;

            if rank_diff == 1 {
                interm_contig_cards.push(*card);
            } else if rank_diff == 0 {
                interm_duplicates.push(*card);
            } else {
                // Ignore intermediates less than 5.
                if interm_contig_cards.len() >= 5 {
                    contig_cards.push(interm_contig_cards.clone());
                    duplicate_cards.extend(interm_duplicates.clone());
                }
                interm_duplicates.clear();
                interm_contig_cards.clear()
            }
        } else {
            interm_contig_cards.push(*card)
        }
    }

    // Add any remaining duplicates or contiguous cards.
    contig_cards.push(interm_contig_cards.clone());
    duplicate_cards.extend(interm_duplicates.clone());

    let mut straights: Vec<Vec<card::Card>> = vec![];
    let duplicate_card_ranks = duplicate_cards.iter().map(|card| card.rank).collect_vec();

    for contig_seq in contig_cards {
        let contig_seq_ranks = contig_seq.iter().map(|card| card.rank).collect_vec();
        let contains_dupe = contig_seq_ranks
            .iter()
            .any(|rank| duplicate_card_ranks.contains(rank));

        // If don't contain duplicates.
        if !contains_dupe {
            // Hand is valid.
            if contig_seq.len() == 5 {
                straights.push(contig_seq.clone());
            } else {
                // Use tuple windows to get contig window of 5-element tuple.
                for hand in contig_seq.iter().tuple_windows::<Hand>() {
                    let vec_hand: Vec<card::Card> =
                        vec![*hand.0, *hand.1, *hand.2, *hand.3, *hand.4];
                    straights.push(vec_hand);
                }
            }
        } else {
            for dupe_card in duplicate_cards.iter() {
                if let Some((i, swappable_card)) = contig_seq
                    .iter()
                    .enumerate()
                    .find(|(_, card)| card.rank == dupe_card.rank)
                {
                    if contig_seq.len() == 5 {
                        // Push the already valid hand.
                        straights.push(contig_seq.clone());

                        // Clone the modified version and remove the dupe card.
                        let mut modified_contig_seq = contig_seq.clone();
                        modified_contig_seq.remove(i);

                        // Based on index, insert or push to account for modded index.
                        match i {
                            0 => modified_contig_seq.insert(0, *swappable_card),
                            5 => modified_contig_seq.push(*swappable_card),
                            _ => modified_contig_seq.insert(i + 1, *swappable_card),
                        }
                        // Add the modified sequence.
                        straights.push(modified_contig_seq);
                    } else {
                        // TODO: Add duplicate swap.
                        // Use tuple windows to get contig window of 5-element tuple.
                        for hand in contig_seq.iter().tuple_windows::<Hand>() {
                            let vec_hand: Vec<card::Card> =
                                vec![*hand.0, *hand.1, *hand.2, *hand.3, *hand.4];
                            straights.push(vec_hand);
                        }
                    }
                }
            }
        }
    }
    println!("{:#?}", straights)
}

pub fn get_flushes(hand: &[card::Card]) -> Vec<Vec<card::Card>> {
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();

    let mut possible_flushes: Vec<Vec<card::Card>> = vec![];
    for suit in suit::Suit::iter() {
        let suit_cards = hand_copy
            .iter()
            .filter_map(|card| if card.suit == suit { Some(*card) } else { None })
            .collect_vec();
        if suit_cards.len() >= 5 {
            possible_flushes.push(suit_cards);
        }
    }

    possible_flushes
}
pub fn get_combos(hand: &[card::Card]) {
    // recursive solution?
    // find cards, pop from vec, and call itself?
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();

    // full house
    // bomb (4 of kind + 1)
    println!("{:?}", get_straights(&hand_copy));
}
