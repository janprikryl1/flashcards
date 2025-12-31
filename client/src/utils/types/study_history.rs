use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, PartialEq, Deserialize)]
pub struct StudyHistoryCreate {
    pub deck_id: i64,
    pub accuracy: u8
}

#[derive(Clone, Serialize, PartialEq, Deserialize)]
pub struct StudyHistory {
    pub id: i64,
    pub user_id: i64,
    pub deck_id: i64,
    pub deck_name: String,
    pub filled_at: String,
    pub accuracy: f32
}