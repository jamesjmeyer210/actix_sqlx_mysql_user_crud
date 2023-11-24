use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use crate::model::{Role, User};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Realm {
    pub id: i32,
    pub name: String,
    pub created_on_utc: DateTime<Utc>,
    pub roles: Vec<Role>,
    pub users: Vec<User>,
}

impl From<&str> for Realm {
    fn from(name: &str) -> Self {
        Realm {
            id: 0,
            name: name.to_string(),
            created_on_utc: Utc::now(),
            roles: Vec::with_capacity(0),
            users: Vec::with_capacity(0)
        }
    }
}

impl<'c> FromRow<'c, SqliteRow> for Realm {
    fn from_row(row: &'c SqliteRow) -> Result<Self, Error> {
        Ok(Realm {
            id: row.get(0),
            name: row.get(1),
            created_on_utc: row.get(2),
            roles: Vec::with_capacity(0),
            users: Vec::with_capacity(0)
        })
    }
}