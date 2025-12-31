use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Serialize, PartialEq, FromRow, Deserialize)]
pub struct StudyHistoryCreateDTO {
    pub deck_id: i64,
    pub accuracy: f32
}


#[derive(Clone, Serialize, PartialEq, FromRow, Deserialize)]
pub struct StudyHistoryDTO {
    pub id: i64,
    pub user_id: i64,
    pub deck_id: i64,
    pub deck_name: String,
    pub filled_at: String,
    pub accuracy: f32
}