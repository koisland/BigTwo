// test_add_seq.json
/*
    *4 plays

    doubles
    - A

    combo
    - straight
        - 9
        - 10
        - J
        - Q
        - K
    - full house
        - triple 5
        - double 3

    singles
    - J

*/

/*

    *5 plays

    doubles
    - 3
    - 5
    - A

    combo
    - straight
        - 9
        - 10
        - J
        - Q
        - K

    singles
    - 5
    - J

*/

use crate::common::{
    card::Card,
    hand::{ComboType, Hand, HandType},
};
use crate::logic::combo::{get_combos, get_dupes};

// Devalue sequential doubles
// Reduce total number of moves
// Use weakest cards in combo if possible.
// If any opponent down to single, prioritize doubles to force pass.
// If any opponent down to low number of cards, prioritize high cards to force pass.
pub fn choose_move(cards: &[Card], prev_hand: &Hand, current_pos: usize, n_cards_left: &[usize]) {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_open_single_start_game() {}

    #[test]
    fn test_open_double_start_game() {}

    #[test]
    fn test_open_combo_start_game() {}

    #[test]
    fn test_respond_single_start_game() {}

    #[test]
    fn test_respond_double_start_game() {}

    #[test]
    fn test_respond_combo_start_game() {}

    #[test]
    fn test_respond_single_end_game() {}

    #[test]
    fn test_respond_double_end_game() {}

    #[test]
    fn test_respond_combo_end_game() {}
}
