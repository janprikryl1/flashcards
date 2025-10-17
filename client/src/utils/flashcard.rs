#[derive(Clone, PartialEq)]
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