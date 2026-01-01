use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct CardCount {
    pub cards: i64,
    pub decks: i64,
}