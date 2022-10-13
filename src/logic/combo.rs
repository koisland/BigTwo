use crate::common::{card::Card, suit::Suit};
use itertools::Itertools;
use std::thread::{self, JoinHandle};
use strum::IntoEnumIterator;

type Hand<'a> = (&'a Card, &'a Card, &'a Card, &'a Card, &'a Card);

pub fn get_dupes(hand: &[Card], size: usize) -> Vec<Vec<Card>> {
    // Sort hand before finding duplicates.
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();

    let mut duplicates: Vec<Vec<Card>> = vec![];
    let mut possible_dupe: Vec<Card> = vec![];

    for card in hand_copy {
        if let Some(last_card) = possible_dupe.last() {
            if last_card.rank == card.rank {
                possible_dupe.push(card)
            } else {
                if possible_dupe.len() >= size {
                    duplicates.push(possible_dupe.clone())
                }
                // Clear the possible duplicates and add the next card.
                possible_dupe.clear();
                possible_dupe.push(card);
            }
        } else {
            possible_dupe.push(card)
        }
    }
    // Add last possible duplicate.
    if possible_dupe.len() >= size {
        duplicates.push(possible_dupe.clone());
    }

    let mut dupe_combs: Vec<Vec<Card>> = vec![];
    for dupe in duplicates {
        for dupe_comb in dupe.iter().combinations(size) {
            let dupe_comb_copy = dupe_comb.iter().map(|card| *card.clone()).collect_vec();
            dupe_combs.push(dupe_comb_copy);
        }
    }
    dupe_combs
}

pub fn get_bombs(hand: &[Card]) -> Vec<Vec<Card>> {
    let hand_copy = hand.to_vec();

    let mut bombs: Vec<Vec<Card>> = vec![];

    let quads = get_dupes(&hand_copy[..], 4);

    // Get all quad card ranks.
    let quad_ranks = quads
        .iter()
        .map(|quad| quad.first().and_then(|card| Some(card.rank)).unwrap())
        .collect_vec();

    if quads.len() != 0 {
        for quad in quads {
            let quad_comb: Vec<Vec<Card>> = hand_copy
                .iter()
                .filter_map(|card| {
                    if !quad_ranks.contains(&card.rank) {
                        let mut quad_copy = quad.clone();
                        quad_copy.push(card.clone());
                        Some(quad_copy)
                    } else {
                        None
                    }
                })
                .collect_vec();

            bombs.extend(quad_comb);
        }
    }

    bombs
}

pub fn get_full_house(hand: &[Card]) -> Vec<Vec<Card>> {
    let hand_copy = hand.to_vec();

    let mut full_houses: Vec<Vec<Card>> = vec![];

    let triples = get_dupes(&hand_copy[..], 3);
    // Store triple cards to avoid using them.
    let triple_cards = triples.iter().flatten().collect_vec();
    for triple in triples.iter() {
        let available_cards = hand_copy
            .iter()
            .filter_map(|card| {
                // Don't allow cards that are used for triples.
                if !triple_cards.contains(&card) {
                    Some(card.clone())
                } else {
                    None
                }
            })
            .collect_vec();
        let doubles = get_dupes(&available_cards[..], 2);
        for double in doubles.iter() {
            let double_clone = double.clone();
            let mut full_house = triple.clone();
            // Merge triple and double to make full house.
            // Add it to full houses.
            full_house.extend(double_clone);
            full_houses.push(full_house)
        }
    }
    full_houses
}

pub fn get_straights(hand: &[Card]) -> Vec<Vec<Card>> {
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();

    let mut contig_cards: Vec<Vec<Card>> = vec![];
    let mut duplicate_cards: Vec<Card> = vec![];
    let mut interm_contig_cards: Vec<Card> = vec![];
    let mut interm_duplicates: Vec<Card> = vec![];

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

    let mut straights: Vec<Vec<Card>> = vec![];
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
                    let vec_hand: Vec<Card> = vec![*hand.0, *hand.1, *hand.2, *hand.3, *hand.4];
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
                        for (idx_diff, hand) in
                            contig_seq.iter().tuple_windows::<Hand>().enumerate()
                        {
                            let vec_hand: Vec<Card> = match i - idx_diff {
                                0 => {
                                    vec![*swappable_card, *hand.1, *hand.2, *hand.3, *hand.4]
                                }
                                1 => {
                                    vec![*hand.0, *swappable_card, *hand.2, *hand.3, *hand.4]
                                }
                                2 => {
                                    vec![*hand.0, *hand.1, *swappable_card, *hand.3, *hand.4]
                                }
                                3 => {
                                    vec![*hand.0, *hand.1, *hand.2, *swappable_card, *hand.4]
                                }
                                _ => {
                                    vec![*hand.0, *hand.1, *hand.2, *hand.3, *swappable_card]
                                }
                            };

                            straights.push(vec_hand);
                        }
                    }
                }
            }
        }
    }
    straights
}

pub fn get_flushes(hand: &[Card]) -> Vec<Vec<Card>> {
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();

    let mut possible_flushes: Vec<Vec<Card>> = vec![];
    for suit in Suit::iter() {
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
pub fn get_combos(hand: &[Card]) -> Vec<Vec<Card>> {
    // recursive solution?
    // find cards, pop from vec, and call itself?

    // let mut handles: Vec<JoinHandle<Vec<Vec<Card>>>> = vec![];
    // let combo_fns: Vec<fn(&[Card]) -> Vec<Vec<Card>>> = vec![
    //     get_straights
    // ];

    // for combo_func in combo_fns {
    //     let hand_copy = hand.to_vec();
    //     let handle = thread::spawn(move || {
    //         let possible_combo = combo_func(&hand_copy[..]);
    //         possible_combo
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     if let Ok(all_combos) = handle.join() {
    //         println!("{:?}", all_combos)
    //     } else {
    //         println!("{}", "No combo. Looking for doubles.")
    //     }
    // }

    let full_houses = get_full_house(&hand[..]);
    full_houses
}
