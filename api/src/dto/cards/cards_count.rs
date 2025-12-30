use serde::Serialize;

#[derive(Clone, Serialize)]
pub(crate) struct CardCount {
    pub(crate) cards: i64,
    pub(crate) decks: i64,
}