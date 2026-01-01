use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}