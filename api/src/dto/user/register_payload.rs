use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct RegisterPayload {
    pub(crate) email: String,
    pub(crate) password: String,
}