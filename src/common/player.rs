use crate::common::card::Card;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Player {
    pub id: usize,
    pub cards: Vec<Card>,
}
