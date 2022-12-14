use crate::common::{
    card::Card, deck::Deck, hand::HandType, player::Player, rank::Rank, stack::CardStack,
    suit::Suit,
};
use crate::logic::choice::choose_move;
use itertools::Itertools;
use regex::Regex;
use std::io;

pub const STARTING_CARD: Card = Card {
    rank: Rank::Three,
    suit: Suit::Diamond,
};

/// Initialize a game by creating a `Deck` of cards
///
fn init(n_players: usize) -> Result<(Deck, Vec<Player>, usize), &'static str> {
    // Generate deck.
    let mut players: Vec<Player> = vec![];
    let new_deck = Deck::new(true);
    let mut starting_player: usize = 0;

    if let Ok(shuffled_deck) = new_deck {
        match shuffled_deck.divide(n_players) {
            Ok(chunks) => {
                for (i, chunk) in chunks.into_iter().enumerate() {
                    let deck_chunk = chunk.clone().into_iter().cloned().collect_vec();

                    // Check if starting player.
                    if deck_chunk.contains(&STARTING_CARD) {
                        starting_player = i;
                    }

                    let player = Player {
                        id: i,
                        cards: deck_chunk,
                    };
                    players.push(player)
                }

                Ok((shuffled_deck.clone(), players, starting_player))
            }
            Err(err_msg) => panic!("{:?}", err_msg),
        }
    } else {
        panic!("Failed to generate deck.")
    }
}

/// Get player index based on turn number and number of players.
fn player_idx(turn_n: usize, n_players: usize) -> usize {
    (turn_n - 1) % n_players
}

/// Start main game loop.
pub fn start(n_players: usize, hotseat: bool) {
    let (_, mut players, mut starting_player) = init(n_players).unwrap();

    let mut pile = CardStack::new();

    // https://dhghomon.github.io/easy_rust/Chapter_63.html
    let user_input_key_msg =
        "- q : Quit\n- p : Pass\n- h : Print this message.\n- s : Sort hand.\n- r : Restart.\n-c : Player a computer move.\n";
    let welcome_msg = format!(
        "\nWelcome to Big 2!\n{user_input_key_msg}\nPlaying against {n_players} players.\n"
    );

    println!("{welcome_msg}");

    let mut user_input = String::new();
    let card_idx_pattern = Regex::new(r"(\d+,*)+").unwrap();
    let mut turn_n: usize = 1;

    // Main game loop.
    loop {
        // First clear the String. Otherwise it will keep adding to it
        user_input.clear();

        // Get current player idx to access cards.
        let curr_player_idx = if turn_n == 1 {
            starting_player
        } else {
            player_idx(turn_n + starting_player, n_players)
        };

        if let Some(prev_hand) = pile.stack.last() {
            // Clear stack as no other player could respond to player's hand.
            // Allow fresh hand.
            if prev_hand.player == curr_player_idx {
                pile.clear();
            }
        }

        println!("Current Turn: {turn_n}");
        if let Some(prev_hand) = pile.stack.last() {
            println!(
                "Current Hand: {:?} (Player {})",
                prev_hand.cards,
                prev_hand.player + 1
            );
        } else {
            println!("Current Hand: None");
        }

        // Format current mode string if combo.
        let curr_mode = if pile.kind == HandType::Combo {
            format!("{:?} ({:?})", pile.kind, pile.combo)
        } else {
            format!("{:?}", pile.kind)
        };

        println!("Current Mode: {:?}\n", curr_mode);
        println!("Your Hand (Player {}):", curr_player_idx + 1);

        if let Some(curr_player) = &players.get(curr_player_idx) {
            for (idx, card) in curr_player.cards.iter().enumerate() {
                println!("{} - {:?}", idx, card)
            }
        }

        // If hotseat, allow user input.
        if (curr_player_idx == 1) | (hotseat) {
            // Get the stdin from the user, and put it in user_input.
            io::stdin().read_line(&mut user_input).unwrap();
        } else {
            // If computer player, use computer move.
            user_input.push('c')
        }

        // Match user input.
        match user_input.trim() {
            "c" => {
                let remaining_cards = players
                    .iter()
                    .map(|player| player.cards.len())
                    .collect_vec();
                let prev_hand = pile.stack.last();
                if let Some(comp_player) = players.get_mut(curr_player_idx) {
                    if let Some((hand, player)) = choose_move(
                        &comp_player.cards,
                        comp_player,
                        prev_hand,
                        curr_player_idx,
                        &remaining_cards,
                    ) {
                        pile.add(&hand.cards, player).unwrap();

                        let new_cards = player
                            .cards
                            .iter()
                            .filter(|card| !hand.cards.contains(&card))
                            .cloned()
                            .collect_vec();

                        if new_cards.is_empty() {
                            println!("Game over.");
                            break;
                        }
                        comp_player.cards = new_cards;
                    }

                    turn_n += 1;
                }
            }
            "q" => {
                println!("See you later!");
                // exit_condition = true;
                break;
            }
            "p" => {
                turn_n += 1;
            }
            "h" => {
                println!("{user_input_key_msg}");
                continue;
            }
            "s" => {
                if let Some(curr_player) = players.get_mut(curr_player_idx) {
                    let mut sorted_hand = curr_player.cards.clone();
                    sorted_hand.sort();
                    curr_player.cards = sorted_hand;
                    continue;
                }
            }
            "r" => {
                let (_, new_players, new_starting_player) = init(n_players).unwrap();
                let new_pile = CardStack::new();
                turn_n = 1;
                starting_player = new_starting_player;
                players = new_players;
                pile = new_pile;
            }
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
                        if let Some(curr_player) = players.get_mut(curr_player_idx) {
                            let mut new_hand = curr_player.cards.clone();

                            // Iter in reverse to not shift indices.
                            let mut playing_hand = vec![];
                            for idx in card_idx.iter().sorted().rev() {
                                if (0..new_hand.len()).contains(idx) {
                                    playing_hand.push(new_hand.remove(*idx))
                                } else {
                                    println!("Index of ({idx}) is not in hand.")
                                }
                            }

                            if turn_n == 1 && !playing_hand.contains(&STARTING_CARD) {
                                println!("First hand must contain the {:?}\n", &STARTING_CARD);
                                continue;
                            };

                            if let Err(err_msg) = pile.add(&playing_hand, curr_player) {
                                println!("Played hand is invalid: {err_msg}\n");
                                continue;
                            } else {
                                if new_hand.is_empty() {
                                    println!("You won!");
                                    break;
                                }
                                // Set player hand to new hand.
                                curr_player.cards = new_hand
                            }
                        }
                        turn_n += 1
                    }
                } else {
                    println!(
                        "Invalid indices ({}). Doesn't match pattern: \\d,\\d,...\n",
                        user_input.trim()
                    );
                }
            }
        }
    }
}
