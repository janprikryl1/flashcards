use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: i64,
    pub question: String,
    pub answer: String,
    pub deck_id: i64,
    pub created_at: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct NewFlashcard {
    pub question: String,
    pub answer: String,
    pub deck_id: i64,
}

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FlashcardPatch {
    pub question: Option<String>,
    pub answer: Option<String>,
    pub deck_id: Option<i64>,
}

#[derive(Clone, PartialEq)]
pub struct StudyFlashcard {
    pub(crate) flashcard: Flashcard,
    pub(crate) last_reviewed: Option<String>,
}
