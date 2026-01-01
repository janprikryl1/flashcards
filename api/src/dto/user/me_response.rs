use serde::Serialize;

#[derive(Serialize)]
pub struct MeResponse {
    pub id: i64,
    pub email: String,
}
