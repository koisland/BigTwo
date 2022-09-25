use crate::common::card::Card;
use crate::common::rank::Rank;
use crate::common::suit::Suit;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

pub fn generate_deck() -> Vec<Card> {
    let mut rng = thread_rng();
    let mut deck_of_cards: Vec<Card> = vec![];

    for suit in Suit::iter() {
        for rank in Rank::iter() {
            let new_card = Card { suit, rank };
            deck_of_cards.push(new_card);
        }
    }

    deck_of_cards.shuffle(&mut rng);
    deck_of_cards
}
