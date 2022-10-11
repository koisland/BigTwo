use itertools::Itertools;
use serde_json::to_writer_pretty;
use std::fs::File;
use std::path::Path;

use crate::common::{card, deck};
use crate::logic::combo;

fn main() {
    let cards_test_file = "test/cards_straights.json";
    if Path::new(cards_test_file).exists() {
        let cards: Vec<card::Card> =
            serde_json::from_reader(&File::open(cards_test_file).unwrap()).unwrap();

        combo::get_combos(&cards);
        // println!("{:#?}", combos)
    } else {
        let shuffled_deck = deck::generate_deck();
        let players = 4;
        let player_card_cnt = shuffled_deck.len() / players;

        let card_chunks = shuffled_deck.into_iter().chunks(player_card_cnt);

        let mut player_cards: Vec<Vec<card::Card>> = vec![];
        for cards in &card_chunks {
            let player_n_cards = cards.collect::<Vec<card::Card>>();
            player_cards.push(player_n_cards);
        }

        to_writer_pretty(&File::create(cards_test_file).unwrap(), &player_cards[0]).unwrap();
    }
}

mod common;
mod logic;
