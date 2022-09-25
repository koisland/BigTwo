use serde_json::to_writer_pretty;
use std::fs::File;
use std::path::Path;

use crate::common::{card, deck};
// use crate::logic::{combo, doubles};

fn main() {
    if Path::new("./cards.json").exists() {
        println!("Card file exists.");
        let cards: Vec<card::Card> =
            serde_json::from_reader(&File::open("cards.json").unwrap()).unwrap();
        // println!("{:#?}", cards);
        for card in cards.iter() {
            println!("{:?}", card.value())
        }

        let mut itr_new_cards = cards.iter();
        let first_card = itr_new_cards.next().unwrap();
        let second_card = itr_new_cards.next().unwrap();
        println!(
            "First Card: {:#?}\nSecond Card: {:#?}",
            first_card, second_card
        );
        println!("Greater than: {}", first_card > second_card);
    } else {
        let mut shuffled_deck = deck::generate_deck();
        let players = 4;
        let player_card_cnt = shuffled_deck.len() / players;

        let mut player_cards: Vec<Vec<card::Card>> = vec![];
        for _ in 0..players {
            let mut player_n_cards: Vec<card::Card> = vec![];
            for _ in 0..player_card_cnt {
                player_n_cards.push(shuffled_deck.pop().unwrap());
            }

            player_cards.push(player_n_cards);
        }

        println!("{:#?}\n", player_cards[0]);
        to_writer_pretty(&File::create("cards.json").unwrap(), &player_cards[0]).unwrap();
    }
}

mod common;
mod logic;
