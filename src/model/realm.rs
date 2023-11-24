use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Realm {
    pub id: i32,
    pub name: String,
    pub created_on_utc: DateTime<Utc>
}

impl From<&'static str> for Realm {
    fn from(name: &'static str) -> Self {
        Realm {
            id: 0,
            name: name.to_string(),
            created_on_utc: Utc::now(),
        }
    }
}

impl<'c> FromRow<'c, SqliteRow> for Realm {
    fn from_row(row: &'c SqliteRow) -> Result<Self, Error> {
        Ok(Realm {
            id: row.get(0),
            name: row.get(1),
            created_on_utc: row.get(2)
        })
    }
}