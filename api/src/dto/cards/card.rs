use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct FlashcardDTO {
    pub id: i64,
    pub question: String,
    pub answer: String,
    pub deck_id: String,
    pub created_at: Option<String>
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct FlashcardCreateDTO {
    pub question: String,
    pub answer: String,
    pub deck_id: String,
    pub created_at: Option<String>
}
