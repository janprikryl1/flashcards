use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,          // user id
    pub exp: usize,        // expiry (unix)
}