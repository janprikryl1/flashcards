use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct MeResponse {
    pub(crate) id: i64,
    pub(crate) email: String,
}
