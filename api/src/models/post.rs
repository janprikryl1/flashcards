use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use std::str::FromStr;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Post {
    pub(crate) title: String,
    pub(crate) content: String,
}
