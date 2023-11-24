// post.rs

use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}
