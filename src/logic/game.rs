use std::io;
use regex::Regex;
use itertools::Itertools;

use crate::common::{card::{Card, self}, deck::Deck, stack::CardStack};


pub fn start(n_players: usize) -> () {
    // Generate deck.
    let mut player_cards: Vec<Vec<Card>> = vec![];
    if let Ok(shuffled_deck) = Deck::new(true) {
        if let Ok(chunks) = shuffled_deck.divide(n_players) {
            for chunk in chunks.into_iter() {
                player_cards.push(chunk.into_iter().cloned().collect_vec())
            }
        }
    }

    // Init card stack.
    let mut pile = CardStack::new();

    // https://dhghomon.github.io/easy_rust/Chapter_63.html
    let user_input_key_msg = "- q : Quit\n- p : Pass\n- h : Print this message.\n- s : Sort hand.\n";
    let welcome_msg = format!("\nWelcome to Big 2!\n{user_input_key_msg}\nPlaying against {n_players} players.\n");

    println!("{}", welcome_msg);

    let mut exit_condition = false;
    let mut user_input = String::new();
    let card_idx_pattern = Regex::new(r"(\d+,*)+").unwrap();
    let mut turn_n: usize = 1;

    while exit_condition != true {
        // First clear the String. Otherwise it will keep adding to it
        user_input.clear();

        println!("Current Turn: {turn_n}");
        if let Some(prev_hand) = pile.stack.last() {
            println!("Current Hand: {:?}", prev_hand);
        } else {
            println!("Current Hand: {}", "None");
        }

        println!("Current Mode: {:?}\n", pile.kind);
        println!("Your Hand:");

        if let Some(curr_hand) = &player_cards.first() {
            for (idx, card) in curr_hand.iter().enumerate() {
                println!("{} - {:?}", idx, card)
            }
        } else {
            println!("Game Over!");
            exit_condition = true
        }

        // Get the stdin from the user, and put it in read_string
        io::stdin().read_line(&mut user_input).unwrap();

        // Match user input.
        match user_input.trim() {
            "q" => {
                println!("See you later!");
                exit_condition = true
            },
            "p" => {
                // TODO: AI move.
                turn_n += 1
            },
            "h" => {
                println!("{}", user_input_key_msg)
            },
            "s" => {
                if let Some(curr_hand) = player_cards.first() {
                    let mut sorted_hand = curr_hand.clone();
                    sorted_hand.sort();
                    player_cards[0] = sorted_hand
                }
            },
            _ => {
                // Search user input for pattern.
                if card_idx_pattern.is_match(user_input.trim()) {
                    let mut card_idx: Vec<usize> = vec![];
                    for idx in user_input.trim().split(',') {
                        if let Ok(parsed_idx) = idx.parse::<usize>() {
                            card_idx.push(parsed_idx)
                        } else {
                            println!("Invalid index. ({idx})\n");
                            continue;
                        }
                    }

                    if !card_idx.is_empty() {
                        // Sort indices
                        card_idx.sort();

                        if let Some(curr_hand) = player_cards.first() {
                            let mut new_hand = curr_hand.clone();

                            // Iter in reverse to not shift indices.
                            let playing_hand = card_idx
                                .iter()
                                .rev()
                                .map(|idx| new_hand.remove(*idx))
                                .collect_vec();

                            if let Err(err_msg) = pile.add(&playing_hand) {
                                println!("Played hand is invalid: {err_msg}\n");
                                continue;
                            } else {
                                // Set player hand to new hand.
                                player_cards[0] = new_hand
                            }

                        }
                        turn_n += 1
                    }

                } else {
                    println!("Invalid indices. Doesn't match pattern: \\d,\\d,...\n");
                }
            }
        }

    }

}
