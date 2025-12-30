use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Serialize, PartialEq, FromRow, Deserialize)]
pub struct DeckCreateDTO {
    pub name: String,
    pub description: String,
    pub color: String,
}


#[derive(Clone, Serialize, PartialEq, FromRow, Deserialize)]
pub struct DeckDTO {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub color: String,
}