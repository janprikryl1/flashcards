use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, FromRow)]
pub struct FlashcardDTO {
    pub id: i64,
    pub question: String,
    pub answer: String,
    pub deck_id: i64,
    pub created_at: Option<String>
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, FromRow)]
pub struct FlashcardCreateDTO {
    pub question: String,
    pub answer: String,
    pub deck_id: i64,
}


#[derive(Clone, PartialEq, Default, Serialize, Deserialize, FromRow)]
pub struct FlashcardPatchDTO {
    pub question: Option<String>,
    pub answer: Option<String>,
    pub deck_id: Option<i64>,
}