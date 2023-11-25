use chrono::{DateTime, Utc};
use super::Role;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::sqlite::SqliteRow;
use uuid::Uuid;
use crate::model::realm::Realm;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: Vec<u8>,
    pub password: Vec<u8>,
    pub email_verified: bool,
    pub phone: Option<Vec<u8>>,
    pub phone_verified: bool,
    pub public_key: Vec<u8>,
    pub groups: Vec<Role>,
    pub realms: Vec<Realm>,
    pub created_on_utc: DateTime<Utc>,
    pub deleted_on_utc: Option<DateTime<Utc>>
}

impl Default for User {
    fn default() -> Self {
        User::new("root", "root@localhost", "")
    }
}

impl<'c> FromRow<'c, SqliteRow> for User {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
            email: row.get(3),
            email_verified: row.get(4),
            phone: row.get(5),
            phone_verified: row.get(6),
            public_key: row.get(7),
            groups: Vec::with_capacity(0),
            realms: Vec::with_capacity(0),
            created_on_utc: row.get(8),
            deleted_on_utc: row.get(9)
        })
    }
}

impl User {
    pub fn new(name: &str, email: &str, password: &str) -> Self
    {
        User {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            password: password.as_bytes().to_vec(),
            email: email.as_bytes().to_vec(),
            email_verified: false,
            phone: None,
            phone_verified: false,
            public_key: Vec::new(),
            groups: Vec::new(),
            realms: Vec::new(),
            created_on_utc: Utc::now(),
            deleted_on_utc: None,
        }
    }
}