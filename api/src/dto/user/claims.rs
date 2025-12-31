use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: i64,          // user id
    pub(crate) exp: usize,        // expiry (unix)
}