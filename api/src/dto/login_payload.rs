use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct LoginPayload {
    pub(crate) email: String,
    pub(crate) password: String,
}