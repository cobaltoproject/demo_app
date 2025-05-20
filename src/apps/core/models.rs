use cobalto_derive::Model;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Model)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
}
