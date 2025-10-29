use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: String,
    pub question: String,
    pub answer: String,
    pub deck_id: String,
    pub created_at: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct NewFlashcard {
    pub question: String,
    pub answer: String,
    pub deck_id: String,
}

#[derive(Clone, PartialEq, Default)]
pub struct FlashcardPatch {
    pub question: Option<String>,
    pub answer: Option<String>,
    pub deck_id: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct FlashcardUpdate {
    pub last_reviewed: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct StudyFlashcard {
    pub(crate) flashcard: Flashcard,
    pub(crate) last_reviewed: Option<String>,
}
