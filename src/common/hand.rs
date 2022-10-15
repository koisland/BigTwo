use itertools::Itertools;

use crate::common::card::Card;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HandType {
    None = 0,
    Single = 1,
    Double = 2,
    Combo = 5,
}

/// Combo types reference: https://www.pagat.com/climbing/bigtwo.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ComboType {
    None,
    Straight,
    Flush,
    FullHouse,
    Bomb,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    kind: HandType,
    combo: ComboType,
}

trait Gauge {
    fn calc_strength(&self) -> usize;
}

trait Parse {
    fn is_valid(hand: &[Card]) -> Result<(HandType, ComboType), &'static str>;
    fn get_combo_type(hand: &[Card]) -> ComboType;
    fn is_flush(hand: &[Card]) -> bool;
    fn is_straight(hand: &[Card]) -> bool;
    fn is_dupe_combo(hand: &[Card], combo_type: ComboType) -> bool;
    fn is_royal_flush(hand: &[Card]) -> bool;
}

impl Hand {
    /// Create a new hand and evaluate if it is valid or not.
    pub fn new(hand: &[Card]) -> Result<Hand, &'static str> {
        let valid_hand = Hand::is_valid(hand);
        if let Ok((hand_type, combo_type)) = valid_hand {
            let new_hand = Hand {
                cards: hand.to_vec(),
                kind: hand_type,
                combo: combo_type,
            };
            Ok(new_hand)
        } else {
            Err(valid_hand.unwrap_err())
        }
    }
}

impl Gauge for Hand {
    fn calc_strength(&self) -> usize {
        todo!()
    }
}

impl Parse for Hand {
    /// Check if a hand is valid based on it's length and if it is a combo, where it is valid or not.
    fn is_valid(hand: &[Card]) -> Result<(HandType, ComboType), &'static str> {
        match hand.len() {
            1 => Ok((HandType::Single, ComboType::None)),
            2 => {
                let is_double = hand
                    .iter()
                    .all(|card| hand.get(0).unwrap().rank == card.rank);
                if is_double {
                    Ok((HandType::Double, ComboType::None))
                } else {
                    Err("Error: Not all cards in double are equal.")
                }
            }
            5 => {
                let combo_type = Hand::get_combo_type(hand);

                // If combo type isn't None.
                if combo_type.ne(&ComboType::None) {
                    Ok((HandType::Combo, combo_type))
                } else {
                    Err("Error: Hand is not a valid 5-card combo.")
                }
            }
            _ => Err("Error: Hand has invalid length."),
        }
    }

    fn get_combo_type(hand: &[Card]) -> ComboType {
        let is_flush = Hand::is_flush(hand);
        let is_straight = Hand::is_straight(hand);
        let is_bomb = Hand::is_dupe_combo(hand, ComboType::Bomb);
        let is_full_house = Hand::is_dupe_combo(hand, ComboType::FullHouse);
        let is_royal_flush = Hand::is_royal_flush(hand);
        let is_straight_flush = is_flush && is_straight;

        if is_royal_flush {
            ComboType::RoyalFlush
        } else if is_straight_flush {
            ComboType::StraightFlush
        } else if is_bomb {
            ComboType::Bomb
        } else if is_full_house {
            ComboType::FullHouse
        } else if is_straight {
            ComboType::Straight
        } else if is_flush {
            ComboType::Flush
        } else {
            ComboType::None
        }
    }

    fn is_flush(hand: &[Card]) -> bool {
        hand.len() == HandType::Combo as usize
            && hand
                .iter()
                .all(|card| card.suit == hand.get(0).unwrap().suit)
    }

    fn is_straight(hand: &[Card]) -> bool {
        hand.len() == HandType::Combo as usize
            && hand
                .iter()
                .sorted()
                .tuple_windows()
                .all(|card_pair: (&Card, &Card)| {
                    (card_pair.1.rank as usize - card_pair.0.rank as usize) == 1
                })
    }

    fn is_dupe_combo(hand: &[Card], combo_type: ComboType) -> bool {
        // Check if valid duplicate card combo type (full house or bomb)
        if let Some(n_dupes) = match combo_type {
            ComboType::Bomb => Some(4),
            ComboType::FullHouse => Some(3),
            _ => None,
        } {
            // Check that hand only contains two unique card ranks.
            if let Some((card_rank_1, card_rank_2)) =
                hand.iter().map(|card| card.rank).unique().collect_tuple()
            {
                let n_card_rank_1 = hand.iter().filter(|card| card.rank == card_rank_1).count();
                let n_card_rank_2 = hand.iter().filter(|card| card.rank == card_rank_2).count();

                // Number of both ranks must equal length of hand.
                // One of ranks must be equal to set num of duplicates for searched combo_type.
                (n_card_rank_1 + n_card_rank_2 == HandType::Combo as usize)
                    && ((n_card_rank_1 == n_dupes) | (n_card_rank_2 == n_dupes))
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_royal_flush(hand: &[Card]) -> bool {
        if Hand::is_flush(hand) {
            let royal_flush_ranks: [usize; 5] = [10, 11, 12, 13, 14];
            hand.iter()
                .sorted()
                .map(|card| card.rank as usize)
                .eq(royal_flush_ranks)
        } else {
            false
        }
    }
}

mod tests {
    use super::{ComboType, Hand, Parse};
    use crate::common::card::Card;
    use serde_json::from_reader;
    use std::fs::File;

    fn get_test_hand(combo_type: ComboType) -> Result<Hand, &'static str> {
        let read_cards: Option<Vec<Card>> = match combo_type {
            ComboType::Straight => {
                from_reader(File::open("./test/hand_straight.json").unwrap()).unwrap()
            }
            ComboType::Flush => from_reader(File::open("./test/hand_flush.json").unwrap()).unwrap(),
            ComboType::FullHouse => {
                from_reader(File::open("./test/hand_full_house.json").unwrap()).unwrap()
            }
            ComboType::Bomb => from_reader(File::open("./test/hand_bomb.json").unwrap()).unwrap(),
            ComboType::StraightFlush => {
                from_reader(File::open("./test/hand_straight_flush.json").unwrap()).unwrap()
            }
            ComboType::RoyalFlush => {
                from_reader(File::open("./test/hand_royal_flush.json").unwrap()).unwrap()
            }
            _ => None,
        };

        if let Some(cards) = read_cards {
            let hand_res = Hand::new(&cards);
            if let Ok(hand) = hand_res {
                Ok(hand)
            } else {
                Err(hand_res.unwrap_err())
            }
        } else {
            Err("Error: Invalid test hand combo type.")
        }
    }
    #[test]
    fn test_create_hand_single() {}

    #[test]
    fn test_create_hand_double() {}

    #[test]
    fn test_create_hand_combo() {
        let new_hand = get_test_hand(ComboType::Bomb);
        if let Err(hand_err) = new_hand {
            panic!("{}", hand_err)
        }
    }

    #[test]
    fn test_get_combo_type() {}

    #[test]
    fn test_is_flush() {
        let test_bomb_res = get_test_hand(ComboType::Bomb);
        let test_flush_res = get_test_hand(ComboType::Flush);
        let test_royal_flush_res = get_test_hand(ComboType::RoyalFlush);

        if let (Ok(hand_bomb), Ok(hand_flush), Ok(hand_royal_flush)) =
            (&test_bomb_res, &test_flush_res, &test_royal_flush_res)
        {
            assert_eq!(Hand::is_flush(&hand_bomb.cards), false);
            assert_eq!(Hand::is_flush(&hand_flush.cards), true);
            assert_eq!(Hand::is_flush(&hand_royal_flush.cards), true);
        } else {
            if let Err(test_bomb_res) = test_bomb_res {
                panic!("{} (Bomb)", test_bomb_res)
            } else if let Err(test_flush_res) = test_flush_res {
                panic!("{} (Flush)", test_flush_res)
            } else if let Err(test_royal_flush_res) = test_royal_flush_res {
                panic!("{} (Royal Flush)", test_royal_flush_res)
            } else {
                ""
            };
        }
    }

    #[test]
    fn test_is_straight() {}

    #[test]
    fn test_is_full_house() {}

    #[test]
    fn test_is_bomb() {}

    #[test]
    fn test_is_royal_flush() {}

    #[test]
    fn test_calculate_strength() {
        let hand_straight = get_test_hand(ComboType::Straight);
        let hand_flush = get_test_hand(ComboType::Flush);
        let hand_full_house = get_test_hand(ComboType::FullHouse);
        let hand_bomb = get_test_hand(ComboType::Bomb);
        let hand_royal_flush = get_test_hand(ComboType::RoyalFlush);
    }
}
