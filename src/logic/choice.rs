// test_add_seq.json
/*
    *4 plays

    doubles
    - A

    combo
    - straight
        - 9
        - 10
        - J
        - Q
        - K
    - full house
        - triple 5
        - double 3

    singles
    - J

*/

/*

    *5 plays

    doubles
    - 3
    - 5
    - A

    combo
    - straight
        - 9
        - 10
        - J
        - Q
        - K

    singles
    - 5
    - J

*/

use crate::common::{
    card::Card,
    hand::{ComboType, Gauge, Hand, HandType},
    player::Player,
};
use crate::logic::combo::{get_combos, get_dupes};
use itertools::Itertools;

/*
0 - [♥4]
1 - [♦2]
2 - [♥9]
3 - [♣3]
4 - [♦7]
5 - [♦9]
6 - [♥7]
7 - [♣A]
8 - [♣K]
9 - [♦J]
10 - [♦Q]
11 - [♦3]
12 - [♠3]
*/

pub fn get_sorted_hands<'a>(hands: &'a [Vec<Card>], player: &Player) -> Vec<(&'a Vec<Card>, f32)> {
    hands
        .iter()
        .filter_map(|hand| {
            if let Ok(hand_strength) = Hand::new(hand, player).and_then(|hand| Ok(hand.strength().unwrap())) {
                Some((hand, hand_strength))
            } else {
                None
            }
        })
        .sorted_by(|(_, dupe_1_val), (_, dupe_2_val)| dupe_1_val.partial_cmp(dupe_2_val).unwrap())
        .collect_vec()
}

fn filter_cards_by_strength(
    cards: &[Vec<Card>],
    player: &Player,
    prev_hand: &Hand,
    omit_cards: &[&Card],
    opponent_close_to_win: bool,
) -> Option<Hand> {
    let possible_hands = cards
        .iter()
        .filter_map(|cards| {
            let hand = Hand::new(cards, player).unwrap();

            let contains_cards_to_omit = cards.iter().any(|card| omit_cards.contains(&card));
            if hand > *prev_hand && !contains_cards_to_omit {
                Some(hand)
            } else {
                None
            }
        })
        .sorted()
        .collect_vec();

    // Play strongest cards if any player is close to winning.
    if !opponent_close_to_win {
        possible_hands.first()
    } else {
        possible_hands.last()
    }.cloned()
}

// Devalue sequential doubles
// Reduce total number of moves
// Use weakest cards in combo if possible.
// If any opponent down to single, prioritize doubles to force pass.
// If any opponent down to low number of cards, prioritize high cards to force pass.
pub fn choose_move<'a>(
    cards: &[Card],
    player: &'a Player,
    prev_hand: &Hand,
    current_pos: usize,
    n_cards_left: &[usize],
) -> Option<(Hand, &'a Player)> {
    // If any player is under some number of cards.
    let opponent_close_to_win = n_cards_left
        .iter()
        .enumerate()
        .any(|(i, n_cards)| *n_cards <= 5 && i != current_pos);

    let mut strongest_hands: Vec<Vec<Card>> = vec![];

    let dupe_combos = get_dupes(cards, 2);
    let five_card_combos = get_combos(cards);

    // println!("Hand: {:?}", cards);
    // println!("Duplicates: {:?}", dupe_combos);
    // println!("Combos: {:?}", five_card_combos);

    // If no player is close to winning, filter out strongest cards to save.
    // If any player close to winning, don't filter out strongest cards.
    let strongest_hand_cards = if !opponent_close_to_win {
        if let Some(dupes) = &dupe_combos {
            let largest_dupes = get_sorted_hands(dupes, player);
            // Consider all duplicates.
            for dupe in largest_dupes {
                strongest_hands.push(dupe.0.to_vec())
            }
        }

        if let Some(combos) = &five_card_combos {
            for (_, possible_combos) in combos.iter() {
                let sorted_combos = get_sorted_hands(possible_combos, player);
                // Only consider largest combo.
                let max_combo = sorted_combos.last().unwrap().0;
                strongest_hands.push(max_combo.to_vec())
            }
        }

        strongest_hands
            .iter()
            .flatten()
            .sorted()
            .dedup()
            .collect_vec()
    } else {
        vec![]
    };

    // println!("Avoiding: {:?}", strongest_hand_cards);
    let possible_hand_to_play: Option<Hand> = match prev_hand.kind {
        HandType::Single => {
            let singles = cards.iter().map(|card| vec![*card]).collect_vec();
            filter_cards_by_strength(
                &singles,
                player,
                prev_hand,
                &strongest_hand_cards,
                opponent_close_to_win,
            )
        }
        HandType::Double => {
            if let Some(dupes) = &dupe_combos {
                filter_cards_by_strength(
                    dupes,
                    player,
                    prev_hand,
                    &strongest_hand_cards,
                    opponent_close_to_win,
                )
            } else {
                None
            }
        }
        HandType::Combo => {
            if let Some(five_card_hands) = &five_card_combos {
                // No cards omitted from hand.
                // Just use lowest combo found.
                five_card_hands
                    .iter()
                    .filter_map(|(_, cards)| {
                        filter_cards_by_strength(
                            cards,
                            player,
                            prev_hand,
                            &[],
                            opponent_close_to_win,
                        )
                    })
                    // Use lowest hand possible to beat hand.
                    .min_by(|combo_1, combo_2| combo_1.cmp(combo_2))
            } else {
                None
            }
        }
        _ => None,
    };

    possible_hand_to_play.map(|hand_to_play| (hand_to_play, player))
}

#[cfg(test)]
mod tests {
    use super::choose_move;
    use crate::common::{
        card::Card,
        hand::{
            tests::{get_test_hand, RelativeStrength},
            ComboType, Hand,
        },
        player::Player,
        rank::Rank,
        suit::Suit,
    };

    #[test]
    fn test_open_single_start_game() {
        let test_seq_file = "test/cards.json";
        let cards: Vec<Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();
        let test_player = Player {
            id: 1,
            cards: cards.clone(),
        };
        let hand_single = Hand::new(
            &vec![Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            }],
            &test_player,
        )
        .unwrap();

        if let Some(chosen_single) = choose_move(&cards, &test_player, &hand_single, 0, &[12, 12]) {
            println!("{:?}", chosen_single)
        }
    }

    #[test]
    fn test_open_double_start_game() {
        let test_seq_file = "test/cards_dupes.json";
        let cards: Vec<Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();
        let test_player = Player {
            id: 1,
            cards: cards.clone(),
        };
        let hand_double = Hand::new(
            &vec![
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Club,
                },
            ],
            &test_player,
        )
        .unwrap();

        if let Some(chosen_double) = choose_move(&cards, &test_player, &hand_double, 0, &[12, 12]) {
            println!("{:?}", chosen_double)
        }
    }

    #[test]
    fn test_open_combo_start_game() {
        let test_seq_file = "test/cards_dupes.json";
        let cards: Vec<Card> =
            serde_json::from_reader(&std::fs::File::open(test_seq_file).unwrap()).unwrap();
        let test_player = Player {
            id: 1,
            cards: cards.clone(),
        };
        let hand_straight = Hand::new(
            &vec![
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Diamond,
                },
            ],
            &test_player,
        )
        .unwrap();

        if let Some(chosen_combo) = choose_move(&cards, &test_player, &hand_straight, 0, &[12, 12])
        {
            println!("{:?}", chosen_combo)
        }
    }

    #[test]
    fn test_respond_single_start_game() {}

    #[test]
    fn test_respond_double_start_game() {}

    #[test]
    fn test_respond_combo_start_game() {}

    #[test]
    fn test_respond_single_end_game() {}

    #[test]
    fn test_respond_double_end_game() {}

    #[test]
    fn test_respond_combo_end_game() {}
}
