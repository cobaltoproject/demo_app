use cobalto_derive::Model;
use sqlx::FromRow;

#[derive(Model, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
}
