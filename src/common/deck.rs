use crate::common::card::Card;
use crate::common::rank::Rank;
use crate::common::suit::Suit;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new(shuffle: bool) -> Result<Deck, ()> {
        let mut cards: Vec<Card> = vec![];

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

        Ok(Deck{cards})
    }

    pub fn divide(&self, n_chunks: usize) -> Result<Vec<Vec<&Card>>, &str> {
        if n_chunks > 52 {
            return Err("Cannot divide deck into greater than 52 chunks.");
        }

        let players = 4;
        let player_card_cnt = self.cards.len() / players;

        let card_chunks = self.cards.iter().chunks(player_card_cnt);

        let mut player_cards: Vec<Vec<&Card>> = vec![];
        for cards in &card_chunks {
            let player_n_cards = cards.collect::<Vec<&Card>>();
            player_cards.push(player_n_cards);
        }

        Ok(player_cards)
    }
}
