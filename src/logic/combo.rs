use crate::common::{card::Card, rank::Rank, suit::Suit};
use itertools::Itertools;
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use strum::IntoEnumIterator;

type TupleHand<'a> = (&'a Card, &'a Card, &'a Card, &'a Card, &'a Card);
type PossibleCombos = Option<Vec<Vec<Card>>>;

pub fn get_dupes(hand: &[Card], size: usize) -> PossibleCombos {
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
            let dupe_comb_copy = dupe_comb.into_iter().copied().collect_vec();
            dupe_combs.push(dupe_comb_copy);
        }
    }
    if dupe_combs.is_empty() {
        None
    } else {
        Some(dupe_combs)
    }
}

pub fn get_bombs(hand: &[Card]) -> PossibleCombos {
    let hand_copy = hand.to_vec();

    let mut bombs: Vec<Vec<Card>> = vec![];

    if let Some(quads) = get_dupes(&hand_copy[..], 4) {
        // Get all quad card ranks.
        let quad_ranks = quads
            .iter()
            .map(|quad| quad.first().map(|card| card.rank).unwrap())
            .collect_vec();

        if !quads.is_empty() {
            for quad in quads {
                let quad_comb: Vec<Vec<Card>> = hand_copy
                    .iter()
                    .filter_map(|card| {
                        if !quad_ranks.contains(&card.rank) {
                            let mut quad_copy = quad.clone();
                            quad_copy.push(*card);
                            Some(quad_copy)
                        } else {
                            None
                        }
                    })
                    .collect_vec();

                bombs.extend(quad_comb);
            }
        }

        Some(bombs)
    } else {
        None
    }
}

pub fn get_full_houses(hand: &[Card]) -> PossibleCombos {
    let hand_copy = hand.to_vec();

    let mut full_houses: Vec<Vec<Card>> = vec![];

    if let Some(triples) = get_dupes(&hand_copy[..], 3) {
        // Store triple cards to avoid using them.
        let triple_cards = triples.iter().flatten().collect_vec();
        for triple in triples.iter() {
            let available_cards = hand_copy
                .iter()
                .filter_map(|card| {
                    // Don't allow cards that are used for triples.
                    if !triple_cards.contains(&card) {
                        Some(*card)
                    } else {
                        None
                    }
                })
                .collect_vec();

            if let Some(doubles) = get_dupes(&available_cards[..], 2) {
                for double in doubles.iter() {
                    let double_clone = double.clone();
                    let mut full_house = triple.clone();
                    // Merge triple and double to make full house.
                    // Add it to full houses.
                    full_house.extend(double_clone);
                    full_houses.push(full_house)
                }
            } else {
                // If no doubles with triple, continue.
                continue;
            }
        }

        // If no full houses, return None.
        if full_houses.is_empty() {
            None
        } else {
            Some(full_houses)
        }
    } else {
        // If no triples, return None.
        None
    }
}

pub fn get_straights(hand: &[Card]) -> PossibleCombos {
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();

    let mut contig_cards: Vec<Vec<Card>> = vec![];
    let mut duplicate_cards: Vec<Card> = vec![];
    let mut interm_contig_cards: Vec<Card> = vec![];
    let mut interm_duplicates: Vec<Card> = vec![];

    for card in hand_copy.iter() {
        // Check last card in intermediate straight.
        if let Some(last_card) = interm_contig_cards.last() {
            let card_rank_value = card.rank as usize;
            let last_card_rank_value = last_card.rank as usize;

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
    let duplicate_card_ranks: HashMap<Rank, Vec<&Card>> = duplicate_cards
        .iter()
        .map(|card| card.rank)
        .unique()
        .map(|rank| {
            (
                rank,
                hand_copy
                    .iter()
                    .filter(|card| card.rank == rank)
                    .collect_vec(),
            )
        })
        .collect();
    // println!("dupes: {:?}", duplicate_card_ranks);

    for contig_seq in contig_cards {
        let contains_dupe = contig_seq
            .iter()
            .map(|card| card.rank)
            .any(|rank| duplicate_card_ranks.keys().contains(&rank));

        // Store the indices of the contiguous sequence of cards that contain duplicate ranks.
        let duplicate_card_idxs: HashMap<Rank, usize> = duplicate_card_ranks
            .iter()
            .filter_map(|(rank, _)| {
                if let Some((idx, _)) = contig_seq
                    .iter()
                    .enumerate()
                    .find(|(_, card)| card.rank == *rank)
                {
                    Some((*rank, idx))
                } else {
                    None
                }
            })
            .collect();

        // If don't contain duplicates.
        if !contains_dupe {
            // Hand is valid.
            if contig_seq.len() == 5 {
                straights.push(contig_seq.clone());
            } else {
                // Use tuple windows to get contig window of 5-element tuple.
                for hand in contig_seq.iter().tuple_windows::<TupleHand>() {
                    let new_hand: Vec<Card> = vec![*hand.0, *hand.1, *hand.2, *hand.3, *hand.4];
                    straights.push(new_hand);
                }
            }
        } else {
            // Take multi cartesian product to get all possible duplicates to swap.
            for swappable_cards in duplicate_card_ranks.values().multi_cartesian_product() {
                let mut new_seq: Vec<Card> = contig_seq.clone();

                for swap_card in swappable_cards {
                    // Get the duplicate card idxs for this sequence to replace.
                    if let Some(idx) = duplicate_card_idxs.get(&swap_card.rank) {
                        new_seq.remove(*idx);
                        new_seq.insert(*idx, **swap_card)
                    }
                }

                if new_seq.len() == 5 {
                    straights.push(new_seq);
                } else {
                    // Use tuple windows to get contig window of 5-element tuple.
                    for hand in new_seq.iter().tuple_windows::<TupleHand>() {
                        let new_hand: Vec<Card> = vec![*hand.0, *hand.1, *hand.2, *hand.3, *hand.4];
                        straights.push(new_hand);
                    }
                }
            }
        }
    }
    if straights.is_empty() {
        None
    } else {
        Some(straights)
    }
}

pub fn get_flushes(hand: &[Card]) -> PossibleCombos {
    let mut hand_copy = hand.to_vec();
    hand_copy.sort();

    let mut possible_flushes: Vec<Vec<Card>> = vec![];
    for suit in Suit::iter() {
        let suit_cards = hand_copy
            .iter()
            .filter_map(|card| if card.suit == suit { Some(*card) } else { None })
            .collect_vec();
        // If len of suit_cards greater than or equal to 5, generate all possible permutations.
        if suit_cards.len() >= 5 {
            for perm in suit_cards.iter().combinations(5) {
                let perm_copy = perm.into_iter().copied().collect_vec();
                possible_flushes.push(perm_copy);
            }
        }
    }
    if possible_flushes.is_empty() {
        None
    } else {
        Some(possible_flushes)
    }
}

/// Generate all possible combos
pub fn get_combos(hand: &[Card]) -> Option<HashMap<&str, Vec<Vec<Card>>>> {
    let mut handles: Vec<JoinHandle<PossibleCombos>> = vec![];

    // Define combo names and combo functions.
    let combo_fn_names = vec!["straights", "full_houses", "bombs", "flushes"];
    let combo_fns: Vec<fn(&[Card]) -> PossibleCombos> =
        vec![get_straights, get_full_houses, get_bombs, get_flushes];

    for combo_func in combo_fns {
        let hand_copy = hand.to_vec();
        let handle = thread::spawn(move || combo_func(&hand_copy));
        handles.push(handle);
    }

    // Create combos hashmap to store possible combos
    let mut combos: HashMap<&str, Vec<Vec<Card>>> = HashMap::new();
    for (handle, combo_name) in handles.into_iter().zip(&combo_fn_names) {
        if let Ok(Some(specific_combo)) = handle.join() {
            combos.insert(combo_name, specific_combo);
        }
    }

    // If any combo has a possible combo, return combos.
    if !combos.is_empty() {
        Some(combos)
    } else {
        None
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_add_combo() {
        let test_seq_file = "test/cards_dupes.json";
        let cards: Vec<Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();

        if let Some(combos) = get_combos(&cards) {
            if let Some(straights) = combos.get("straights") {
                println!("Straights:\n{:?}", straights);
                println!("Number of straights: {}\n", straights.len());
                assert_eq!(straights.len(), 8)
            };
            if let Some(flushes) = combos.get("flushes") {
                println!("Flushes:\n{:?}", flushes);
                println!("Number of flushes: {}\n", flushes.len());
                assert_eq!(flushes.len(), 1)
            };
            if let Some(bombs) = combos.get("bombs") {
                println!("Bombs:\n{:?}", bombs);
                println!("Number of bombs: {}\n", bombs.len());
                assert_eq!(bombs.len(), 9)
            };
            if let Some(full_houses) = combos.get("full_houses") {
                println!("Full Houses:\n{:?}", full_houses);
                println!("Number of full houses: {}\n", full_houses.len());
                assert_eq!(full_houses.len(), 5)
            };
        }
    }
}
