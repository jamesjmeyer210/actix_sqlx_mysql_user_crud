use serde::{Deserialize, Serialize};

use sqlx::{FromRow, Row};
use sqlx::sqlite::SqliteRow;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Role {
    pub id: i32,
    pub max: Option<i32>,
    pub name: String,
}

impl<'c> FromRow<'c, SqliteRow> for Role {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Role {
            id: row.get(0),
            max: row.get(1),
            name: row.get(2),
        })
    }
}
