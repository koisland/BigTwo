use crate::common::card::Card;

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    None,
    Single,
    Double,
    Combo,
}

/// Combo types reference: https://www.pagat.com/climbing/bigtwo.html
#[derive(Debug, PartialEq, Eq)]
pub enum ComboType {
    None,
    Straight,
    Flush,
    FullHouse,
    Bomb,
    StraightFlush,
    RoyalFlush,
}

pub struct Hand {
    cards: Vec<Card>,
    kind: HandType,
    combo: ComboType,
    strength: usize,
}

trait Gauge {
    fn calc_strength(hand: &[Card]) -> usize;
}

trait Parse {
    fn is_valid(hand: &[Card]) -> Result<(HandType, ComboType), &'static str>;
    fn get_combo_type(hand: &[Card]) -> Result<ComboType, &'static str>;
    fn is_flush(hand: Vec<Card>) -> bool;
    fn is_straight(hand: Vec<Card>) -> bool;
    fn is_full_house(hand: Vec<Card>) -> bool;
    fn is_bomb(hand: Vec<Card>) -> bool;
    fn is_straight_flush(hand: Vec<Card>) -> bool;
    fn is_royal_flush(hand: Vec<Card>) -> bool;
}

impl Hand {
    /// Create a new hand and evaluate if it is valid or not.
    pub fn new(hand: &[Card]) -> Result<Hand, &'static str> {
        let valid_hand = Hand::is_valid(&hand);
        if let Ok((hand_type, combo_type)) = valid_hand {
            let new_hand = Hand {
                cards: hand.to_vec(),
                kind: hand_type,
                combo: combo_type,
                strength: Hand::calc_strength(hand),
            };
            Ok(new_hand)
        } else {
            Err(valid_hand.unwrap_err())
        }
    }
}

impl Gauge for Hand {
    fn calc_strength(hand: &[Card]) -> usize {
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
                    return Err("Error: Not all cards in double are equal.");
                }
            }
            5 => {
                if let Ok(combo_type) = Hand::get_combo_type(hand) {
                    Ok((HandType::Combo, combo_type))
                } else {
                    Err("Error: Hand is not a valid 5-card combo.")
                }
            }
            _ => return Err("Error: Hand has invalid length."),
        }
    }

    fn get_combo_type(hand: &[Card]) -> Result<ComboType, &'static str> {
        todo!()
    }

    fn is_flush(hand: Vec<Card>) -> bool {
        todo!()
    }

    fn is_straight(hand: Vec<Card>) -> bool {
        todo!()
    }

    fn is_full_house(hand: Vec<Card>) -> bool {
        todo!()
    }

    fn is_bomb(hand: Vec<Card>) -> bool {
        todo!()
    }

    fn is_straight_flush(hand: Vec<Card>) -> bool {
        todo!()
    }

    fn is_royal_flush(hand: Vec<Card>) -> bool {
        todo!()
    }
}
