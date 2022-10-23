use crate::common::{card::Card, error::DeckError, rank::Rank, suit::Suit};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(shuffle: bool) -> Result<Deck, ()> {
        let mut cards: Vec<Card> = Vec::with_capacity(52);

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                let new_card = Card { suit, rank };
                cards.push(new_card);
            }
        }

        if shuffle {
            let mut rng = thread_rng();
            cards.shuffle(&mut rng);
        }

        Ok(Deck { cards })
    }

    pub fn divide(&self, n_chunks: usize) -> Result<Vec<Vec<&Card>>, DeckError> {
        // TODO: Implement odd n-players and 3 of diamonds rule.
        if n_chunks > 52 {
            let err_msg = format!("Deck cannot have greater than 52 chunks. ({}) ", n_chunks);
            return Err(DeckError::InvalidChunks(err_msg));
        }

        let player_card_cnt = self.cards.len() / n_chunks;

        let card_chunks = self.cards.iter().chunks(player_card_cnt);

        let mut player_cards: Vec<Vec<&Card>> = vec![];
        for cards in &card_chunks {
            let player_n_cards = cards.collect::<Vec<&Card>>();
            player_cards.push(player_n_cards);
        }

        Ok(player_cards)
    }
}
