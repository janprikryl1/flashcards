use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Deck {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub color: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct DeckCreate {
    pub name: String,
    pub description: String,
    pub color: String,
}