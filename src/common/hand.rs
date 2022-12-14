use crate::common::{card::Card, error::HandError, player::Player, rank::Rank, suit::Suit};
use itertools::Itertools;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{hash_map::Entry::Vacant, HashMap};
use std::f32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HandType {
    None = 0,
    Single = 1,
    Double = 2,
    Combo = 5,
}

/// Combo types reference: https://www.pagat.com/climbing/bigtwo.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ComboType {
    None = 0,
    Straight = 1,
    Flush = 3,
    FullHouse = 5,
    Bomb = 7,
    StraightFlush = 9,
    RoyalFlush = 11,
}

/// Enum for function defintions.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CardFilter {
    Strongest,
    Weakest,
    MostFrequentRanks,
    LeastFrequentRanks,
    MostFrequentSuits,
    LeastFrequentSuits,
}

/// A group of `Card`s of some `kind`.
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub kind: HandType,
    pub combo: ComboType,
    pub player: usize,
}

pub trait Gauge {
    const STRONGEST_FILTER: [CardFilter; 1];
    const FREQ_STRONGEST_FILTER: [CardFilter; 2];
    fn invalid_hand_err_msg(&self) -> String;
    fn empty_combos_err_msg(&self, filters: &[CardFilter]) -> String;
    /// Calculates the strength of a `Hand`.
    fn strength(&self) -> Result<f32, HandError>;
}

pub trait Parse {
    /// Retrives a set of `Card`s from `Hand` based on a series of `CardFilter` conditions.
    fn get_cards(&self, filters: &[CardFilter]) -> Option<Vec<Card>>;
    fn ranks(&self, opt_cards: Option<&[Card]>) -> HashMap<Rank, usize>;
    fn suits(&self, opt_cards: Option<&[Card]>) -> HashMap<Suit, usize>;
}

pub trait Validate {
    fn is_valid(hand: &[Card]) -> Result<(HandType, ComboType), &'static str>;
    fn is_combo_type(hand: &[Card]) -> ComboType;
    fn is_flush(hand: &[Card]) -> bool;
    fn is_straight(hand: &[Card]) -> bool;
    fn is_dupe_combo(hand: &[Card], combo_type: ComboType) -> bool;
    fn is_royal_flush(hand: &[Card]) -> bool;
}

impl Hand {
    /// Create a new hand and evaluates if it is valid or not.
    pub fn new(hand: &[Card], player: &Player) -> Result<Hand, &'static str> {
        let valid_hand = Hand::is_valid(hand);
        if let Ok((hand_type, combo_type)) = valid_hand {
            let new_hand = Hand {
                cards: hand.to_vec(),
                kind: hand_type,
                combo: combo_type,
                player: player.id,
            };
            Ok(new_hand)
        } else {
            Err(valid_hand.unwrap_err())
        }
    }
}

// TODO: Implement trait for deck.
impl Parse for Hand {
    fn get_cards(&self, filters: &[CardFilter]) -> Option<Vec<Card>> {
        let mut filtered_cards = Some(self.cards.clone());

        for filter_opt in filters.iter() {
            // Exit if no cards left.
            filtered_cards.as_ref()?;

            // Update filtered cards on iteration of filters.
            filtered_cards = match filter_opt {
                CardFilter::Strongest => filtered_cards?
                    .into_iter()
                    .reduce(|c1, c2| c1.max(c2))
                    .map(|strongest_card| vec![strongest_card]),
                CardFilter::Weakest => filtered_cards?
                    .into_iter()
                    .reduce(|c1, c2| c1.min(c2))
                    .map(|weakest_card| vec![weakest_card]),
                // TODO: Simplify and separate repeat code.
                CardFilter::MostFrequentSuits => {
                    let remaining_cards = filtered_cards?;
                    let suit_cnts = self.suits(Some(&remaining_cards));
                    let most_freq_suit = suit_cnts
                        .into_iter()
                        .max_by(|(_, rank_1_cnt), (_, rank_2_cnt)| rank_1_cnt.cmp(rank_2_cnt));
                    if let Some((most_freq_suit, _)) = most_freq_suit {
                        let most_freq_suit_cards = remaining_cards
                            .into_iter()
                            .filter(|card| card.suit == most_freq_suit)
                            .collect_vec();
                        Some(most_freq_suit_cards)
                    } else {
                        None
                    }
                }
                CardFilter::LeastFrequentSuits => {
                    let remaining_cards = filtered_cards?;
                    let suit_cnts = self.suits(Some(&remaining_cards));
                    let least_freq_suit = suit_cnts
                        .into_iter()
                        .min_by(|(_, rank_1_cnt), (_, rank_2_cnt)| rank_1_cnt.cmp(rank_2_cnt));
                    if let Some((least_freq_suit, _)) = least_freq_suit {
                        let least_freq_suit_cards = remaining_cards
                            .into_iter()
                            .filter(|card| card.suit == least_freq_suit)
                            .collect_vec();
                        Some(least_freq_suit_cards)
                    } else {
                        None
                    }
                }
                CardFilter::MostFrequentRanks => {
                    let remaining_cards = filtered_cards?;
                    let rank_cnts = self.ranks(Some(&remaining_cards));
                    let most_freq_rank = rank_cnts
                        .into_iter()
                        .max_by(|(_, rank_1_cnt), (_, rank_2_cnt)| rank_1_cnt.cmp(rank_2_cnt));
                    if let Some((most_freq_rank, _)) = most_freq_rank {
                        let most_freq_rank_cards = remaining_cards
                            .into_iter()
                            .filter(|card| card.rank == most_freq_rank)
                            .collect_vec();
                        Some(most_freq_rank_cards)
                    } else {
                        None
                    }
                }
                CardFilter::LeastFrequentRanks => {
                    let remaining_cards = filtered_cards?;
                    let rank_cnts = self.ranks(Some(&remaining_cards));
                    let least_freq_rank = rank_cnts
                        .into_iter()
                        .min_by(|(_, rank_1_cnt), (_, rank_2_cnt)| rank_1_cnt.cmp(rank_2_cnt));
                    if let Some((least_freq_rank, _)) = least_freq_rank {
                        let least_freq_rank_cards = remaining_cards
                            .into_iter()
                            .filter(|card| card.rank == least_freq_rank)
                            .collect_vec();
                        Some(least_freq_rank_cards)
                    } else {
                        None
                    }
                }
            }
        }
        filtered_cards
    }

    fn ranks(&self, opt_cards: Option<&[Card]>) -> HashMap<Rank, usize> {
        let mut rank_cnts: HashMap<Rank, usize> = HashMap::new();
        // If no opt_cards provided, use instances cards.
        let cards = opt_cards.unwrap_or(&self.cards);
        for card in cards.iter() {
            if let Vacant(rank_cnt_entry) = rank_cnts.entry(card.rank) {
                rank_cnt_entry.insert(0);
            } else if let Some(rank_cnt) = rank_cnts.get_mut(&card.rank) {
                *rank_cnt += 1
            }
        }
        rank_cnts
    }

    fn suits(&self, opt_cards: Option<&[Card]>) -> HashMap<Suit, usize> {
        let mut suit_cnts: HashMap<Suit, usize> = HashMap::new();
        // If no opt_cards provided, use instances cards.
        let cards = opt_cards.unwrap_or(&self.cards);
        for card in cards.iter() {
            if let Vacant(suit_cnt_entry) = suit_cnts.entry(card.suit) {
                suit_cnt_entry.insert(0);
            } else if let Some(suit_cnt) = suit_cnts.get_mut(&card.suit) {
                *suit_cnt += 1
            }
        }
        suit_cnts
    }
}

impl Gauge for Hand {
    fn invalid_hand_err_msg(&self) -> String {
        format!(
            "Error: Invalid hand ({:?}) of {:?}s.",
            self.cards, self.kind
        )
    }

    fn empty_combos_err_msg(&self, filters: &[CardFilter]) -> String {
        format!(
            "Error: Card filters ({:?}) unable to get {:?} from cards ({:?}).",
            filters, self.kind, self.cards
        )
    }

    const FREQ_STRONGEST_FILTER: [CardFilter; 2] =
        [CardFilter::MostFrequentRanks, CardFilter::Strongest];

    const STRONGEST_FILTER: [CardFilter; 1] = [CardFilter::Strongest];

    fn strength(&self) -> Result<f32, HandError> {
        match self.kind {
            HandType::Single => {
                // Return card's base value.
                self.cards.first().map_or(
                    Err(HandError::InvalidHand(self.invalid_hand_err_msg())),
                    |card| Ok(card.value()),
                )
            }
            HandType::Double => {
                if let Some(strongest_card) = self.get_cards(&Hand::STRONGEST_FILTER) {
                    // Multiply by 2.0 for doubles.
                    strongest_card.last().map_or(
                        Err(HandError::InvalidHand(self.invalid_hand_err_msg())),
                        |last_card| Ok(last_card.value() * 2.0),
                    )
                } else {
                    Err(HandError::InvalidHand(
                        self.empty_combos_err_msg(&Hand::STRONGEST_FILTER),
                    ))
                }
            }
            HandType::Combo => {
                // Lowest card value is 1.1 (3 of Diamonds), highest is 13.4 (2 of Spades)
                let hand_strength_multiplier_res: Result<f32, HandError> = match self.combo {
                    ComboType::Straight
                    | ComboType::Flush
                    | ComboType::StraightFlush
                    | ComboType::RoyalFlush => {
                        if let Some(strongest_card) = self.get_cards(&Hand::STRONGEST_FILTER) {
                            let combo_multiplier = (self.combo as usize) as f32;
                            strongest_card
                                .last()
                                .map_or(Err(HandError::InvalidHand("".to_string())), |card| {
                                    Ok(card.value().powf(combo_multiplier))
                                })
                        } else {
                            Err(HandError::InvalidHand(
                                self.empty_combos_err_msg(&Hand::STRONGEST_FILTER),
                            ))
                        }
                    }
                    ComboType::FullHouse | ComboType::Bomb => {
                        if let Some(dupes) = self.get_cards(&Hand::FREQ_STRONGEST_FILTER) {
                            let combo_multiplier = (self.combo as usize) as f32;
                            dupes.iter().max().map_or(
                                Err(HandError::InvalidHand(
                                    self.empty_combos_err_msg(&Hand::FREQ_STRONGEST_FILTER),
                                )),
                                // Multiply by 4.0 to ensure that flushes are weaker than full-houses.
                                |strongest_card| {
                                    Ok(strongest_card.value().powf(combo_multiplier))
                                },
                            )
                        } else {
                            Err(HandError::InvalidHand(
                                self.empty_combos_err_msg(&Hand::FREQ_STRONGEST_FILTER),
                            ))
                        }
                    }
                    _ => Err(HandError::InvalidHand(
                        "Error: Invalid combo type.".to_string(),
                    )),
                };

                if let Ok(hand_strength_multiplier) = hand_strength_multiplier_res {
                    // Multiply by 5.0 for combo and raise combo to power of multiplier.
                    return Ok(hand_strength_multiplier * 5.0);
                } else {
                    return Err(hand_strength_multiplier_res.unwrap_err());
                }
            }
            HandType::None => Err(HandError::InvalidHand(
                "Error: Cannot calculate hand strength for invalid/empty hand.".to_string(),
            )),
        }
    }
}

impl Ord for Hand {
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare hands based on value.
        if self.kind != other.kind {
            panic!("Error: Unable to compare hands of different kind.")
        }
        if let (Ok(hand_1_strength), Ok(hand_2_strength)) = (self.strength(), other.strength()) {
            hand_1_strength.partial_cmp(&hand_2_strength).unwrap()
        } else {
            panic!("Error: Unable to calculate hand strengths.")
        }
    }
}

impl PartialOrd for Hand {
    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        // Pattern `Some(Less | Eq)` optimizes worse than negating `None | Some(Greater)`.
        // FIXME: The root cause was fixed upstream in LLVM with:
        // https://github.com/llvm/llvm-project/commit/9bad7de9a3fb844f1ca2965f35d0c2a3d1e11775
        // Revert this workaround once support for LLVM 12 gets dropped.
        !matches!(self.partial_cmp(other), None | Some(Greater))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Validate for Hand {
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
                    Err("Error: Not all cards in double are equal rank.")
                }
            }
            5 => {
                let combo_type = Hand::is_combo_type(hand);

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

    fn is_combo_type(hand: &[Card]) -> ComboType {
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

                // Number of both ranks must Normal length of hand.
                // One of ranks must be Normal to set num of duplicates for searched combo_type.
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
            let royal_flush_ranks: [usize; 5] = [8, 9, 10, 11, 12];
            hand.iter()
                .sorted()
                .map(|card| card.rank as usize)
                .eq(royal_flush_ranks)
        } else {
            false
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{ComboType, Gauge, Hand, Parse, Validate};
    use crate::common::{card::Card, player::Player, rank::Rank, suit::Suit};
    use serde_json::from_reader;
    use std::fs::File;

    #[derive(Debug, PartialEq, Eq)]
    pub enum RelativeStrength {
        Normal,
        Weaker,
        Stronger,
    }

    pub fn get_test_hand(
        test_player: &Player,
        combo_type: ComboType,
        rel_strength: RelativeStrength,
    ) -> Result<Hand, &'static str> {
        let read_cards: Option<Vec<Card>> = match combo_type {
            ComboType::Straight => {
                let test_file = match rel_strength {
                    RelativeStrength::Normal => "./test/hand_straight.json",
                    RelativeStrength::Stronger => "./test/hand_straight_stronger.json",
                    _ => "./test/hand_straight.json",
                };
                from_reader(File::open(test_file).unwrap()).unwrap()
            }
            ComboType::Flush => {
                let test_file = match rel_strength {
                    RelativeStrength::Normal => "./test/hand_flush.json",
                    RelativeStrength::Weaker => "./test/hand_flush_weaker.json",
                    _ => "./test/hand_flush.json",
                };
                from_reader(File::open(test_file).unwrap()).unwrap()
            }
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
            let hand_res = Hand::new(&cards, &test_player);
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
    fn test_create_hand_single() {
        let cards: Vec<Card> = from_reader(File::open("./test/hand_single.json").unwrap()).unwrap();
        let test_player = Player {
            id: 1,
            cards: cards.clone(),
        };
        if let Err(hand_err) = Hand::new(&cards, &test_player) {
            panic!("{}", hand_err)
        }
    }

    #[test]
    fn test_create_hand_double() {
        let cards: Vec<Card> = from_reader(File::open("./test/hand_double.json").unwrap()).unwrap();
        let test_player = Player {
            id: 1,
            cards: cards.clone(),
        };
        if let Err(hand_err) = Hand::new(&cards, &test_player) {
            panic!("{}", hand_err)
        }
    }

    #[test]
    fn test_create_hand_combo() {
        let test_player = Player {
            id: 1,
            cards: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            }],
        };
        if let Err(hand_err) =
            get_test_hand(&test_player, ComboType::Bomb, RelativeStrength::Normal)
        {
            panic!("{}", hand_err)
        }
    }

    #[test]
    fn test_is_flush() {
        let test_player = Player {
            id: 1,
            cards: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            }],
        };
        let test_bomb_res = get_test_hand(&test_player, ComboType::Bomb, RelativeStrength::Normal);
        let test_flush_res =
            get_test_hand(&test_player, ComboType::Flush, RelativeStrength::Normal);
        let test_royal_flush_res = get_test_hand(
            &test_player,
            ComboType::RoyalFlush,
            RelativeStrength::Normal,
        );

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
            } else {
                panic!("{} (Royal Flush)", test_royal_flush_res.unwrap_err())
            };
        }
    }

    #[test]
    fn test_is_straight() {
        let test_player = Player {
            id: 1,
            cards: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            }],
        };
        let test_bomb_res = get_test_hand(&test_player, ComboType::Bomb, RelativeStrength::Normal);
        let test_straight_res =
            get_test_hand(&test_player, ComboType::Straight, RelativeStrength::Normal);
        let test_royal_flush_res = get_test_hand(
            &test_player,
            ComboType::RoyalFlush,
            RelativeStrength::Normal,
        );

        if let (Ok(hand_bomb), Ok(hand_straight), Ok(hand_royal_flush)) =
            (&test_bomb_res, &test_straight_res, &test_royal_flush_res)
        {
            assert_eq!(Hand::is_straight(&hand_bomb.cards), false);
            assert_eq!(Hand::is_straight(&hand_straight.cards), true);
            assert_eq!(Hand::is_straight(&hand_royal_flush.cards), true);
        } else {
            if let Err(test_bomb_err) = test_bomb_res {
                panic!("{} (Bomb)", test_bomb_err)
            } else if let Err(test_straight_err) = test_straight_res {
                panic!("{} (Straight)", test_straight_err)
            } else {
                panic!("{} (Royal Flush)", test_royal_flush_res.unwrap_err())
            };
        }
    }

    #[test]
    fn test_is_full_house() {
        let test_player = Player {
            id: 1,
            cards: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            }],
        };
        let test_bomb_res = get_test_hand(&test_player, ComboType::Bomb, RelativeStrength::Normal);
        let test_full_house_res =
            get_test_hand(&test_player, ComboType::FullHouse, RelativeStrength::Normal);

        if let (Ok(hand_bomb), Ok(hand_full_house)) = (&test_bomb_res, &test_full_house_res) {
            assert_eq!(
                Hand::is_dupe_combo(&hand_bomb.cards, ComboType::FullHouse),
                false
            );
            assert_eq!(
                Hand::is_dupe_combo(&hand_full_house.cards, ComboType::FullHouse),
                true
            );
        } else {
            if let Err(test_bomb_err) = test_bomb_res {
                panic!("{} (Bomb)", test_bomb_err)
            } else {
                panic!("{} (FullHouse)", test_full_house_res.unwrap_err())
            };
        }
    }

    #[test]
    fn test_is_bomb() {
        let test_player = Player {
            id: 1,
            cards: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            }],
        };
        let test_bomb_res = get_test_hand(&test_player, ComboType::Bomb, RelativeStrength::Normal);
        let test_full_house_res =
            get_test_hand(&test_player, ComboType::FullHouse, RelativeStrength::Normal);

        if let (Ok(hand_bomb), Ok(hand_full_house)) = (&test_bomb_res, &test_full_house_res) {
            assert_eq!(Hand::is_dupe_combo(&hand_bomb.cards, ComboType::Bomb), true);
            assert_eq!(
                Hand::is_dupe_combo(&hand_full_house.cards, ComboType::Bomb),
                false
            );
        } else {
            if let Err(test_bomb_err) = test_bomb_res {
                panic!("{} (Bomb)", test_bomb_err)
            } else {
                panic!("{} (FullHouse)", test_full_house_res.unwrap_err())
            };
        }
    }

    #[test]
    fn test_is_royal_flush() {
        let test_player = Player {
            id: 1,
            cards: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            }],
        };
        let test_bomb_res = get_test_hand(&test_player, ComboType::Bomb, RelativeStrength::Normal);
        let test_straight_res =
            get_test_hand(&test_player, ComboType::Straight, RelativeStrength::Normal);
        let test_royal_flush_res = get_test_hand(
            &test_player,
            ComboType::RoyalFlush,
            RelativeStrength::Normal,
        );

        if let (Ok(hand_bomb), Ok(hand_straight), Ok(hand_royal_flush)) =
            (&test_bomb_res, &test_straight_res, &test_royal_flush_res)
        {
            assert_eq!(Hand::is_royal_flush(&hand_bomb.cards), false);
            assert_eq!(Hand::is_royal_flush(&hand_straight.cards), false);
            assert_eq!(Hand::is_royal_flush(&hand_royal_flush.cards), true);
        } else {
            if let Err(test_bomb_err) = test_bomb_res {
                panic!("{} (Bomb)", test_bomb_err)
            } else if let Err(test_straight_err) = test_straight_res {
                panic!("{} (Straight)", test_straight_err)
            } else {
                panic!("{} (Royal Flush)", test_royal_flush_res.unwrap_err())
            };
        }
    }

    #[test]
    fn test_calculate_strength() {
        let test_player = Player {
            id: 1,
            cards: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            }],
        };
        let hand_straight =
            get_test_hand(&test_player, ComboType::Straight, RelativeStrength::Normal);
        let hand_flush = get_test_hand(&test_player, ComboType::Flush, RelativeStrength::Normal);
        let hand_full_house =
            get_test_hand(&test_player, ComboType::FullHouse, RelativeStrength::Normal);
        let hand_bomb = get_test_hand(&test_player, ComboType::Bomb, RelativeStrength::Normal);
        let hand_straight_flush = get_test_hand(
            &test_player,
            ComboType::StraightFlush,
            RelativeStrength::Normal,
        );
        let hand_royal_flush = get_test_hand(
            &test_player,
            ComboType::RoyalFlush,
            RelativeStrength::Normal,
        );

        if let Ok(straight) = hand_straight {
            if let Ok(straight_strength) = straight.strength() {
                assert_eq!(27.0, straight_strength)
            }
        };
        if let Ok(flush) = hand_flush {
            if let Ok(flush_strength) = flush.strength() {
                assert_eq!(2026.1201, flush_strength)
            }
        };
        if let Ok(full_house) = hand_full_house {
            if let Ok(full_house_strength) = full_house.strength() {
                assert_eq!(22958.254, full_house_strength)
            }
        };
        if let Ok(bomb) = hand_bomb {
            if let Ok(bomb_strength) = bomb.strength() {
                assert_eq!(669462.6, bomb_strength)
            }
        };
        if let Ok(straight_flush) = hand_straight_flush {
            if let Ok(straight_flush_strength) = straight_flush.strength() {
                assert_eq!(19521532.0, straight_flush_strength)
            }
        };
        if let Ok(royal_flush) = hand_royal_flush {
            if let Ok(royal_flush_strength) = royal_flush.strength() {
                assert_eq!(5328542400000.0, royal_flush_strength)
            }
        };
    }

    #[test]
    fn test_combo_cmp() {
        let test_player = Player {
            id: 1,
            cards: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            }],
        };
        let hand_straight =
            get_test_hand(&test_player, ComboType::Straight, RelativeStrength::Normal);
        let hand_straight_stronger = get_test_hand(
            &test_player,
            ComboType::Straight,
            RelativeStrength::Stronger,
        );
        let hand_flush = get_test_hand(&test_player, ComboType::Flush, RelativeStrength::Normal);
        let hand_flush_weaker =
            get_test_hand(&test_player, ComboType::Flush, RelativeStrength::Weaker);
        let hand_full_house =
            get_test_hand(&test_player, ComboType::FullHouse, RelativeStrength::Normal);

        if let (Ok(straight), Ok(straight_stronger)) = (hand_straight, hand_straight_stronger) {
            assert_eq!(straight_stronger > straight, true)
        };
        if let (Ok(flush), Ok(flush_weaker)) = (&hand_flush, hand_flush_weaker) {
            assert_eq!(flush_weaker < *flush, true)
        };
        if let (Ok(full_house), Ok(flush)) = (hand_full_house, hand_flush) {
            assert_eq!(full_house > flush, true)
        };
    }

    #[test]
    #[should_panic]
    fn test_invalid_combo_cmp() {
        let single_cards = vec![Card {
            rank: Rank::Ace,
            suit: Suit::Club,
        }];
        let test_player = Player {
            id: 1,
            cards: single_cards.clone(),
        };
        let hand_straight =
            get_test_hand(&test_player, ComboType::Straight, RelativeStrength::Normal);

        let hand_single = Hand::new(&single_cards, &test_player);

        let _ = hand_single > hand_straight;
    }
}
