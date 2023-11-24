use super::Role;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::sqlite::SqliteRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: Vec<u8>,
    pub email_verified: bool,
    pub phone: Vec<u8>,
    pub phone_verified: bool,
    pub public_key: Vec<u8>,
    pub groups: Vec<Role>,
}

impl<'c> FromRow<'c, SqliteRow> for User {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            email_verified: row.get(3),
            phone: row.get(4),
            phone_verified: row.get(5),
            public_key: row.get(6),
            groups: Vec::with_capacity(0),
        })
    }
}

impl User {
    pub fn new(name: &str, email: &str, phone: &str) -> Self
    {
        User {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            email: email.as_bytes().to_vec(),
            email_verified: false,
            phone: phone.as_bytes().to_vec(),
            phone_verified: false,
            public_key: Vec::new(),
            groups: Vec::new(),
        }
    }
}