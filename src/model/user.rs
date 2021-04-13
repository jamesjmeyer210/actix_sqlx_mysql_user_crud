use super::Group;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};

// TODO: Try to use the #[derive(sqlx::FromRow)] here.
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub groups: Vec<Group>,
}

// impl<'c> FromRow<'c, MySqlRow> for User {
//     fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
//         Ok(User {
//             id: row.get(0),
//             name: row.get(1),
//             email: row.get(2),
//             groups: Vec::with_capacity(0),
//         })
//     }
// }

impl FromRow<'_, MySqlRow> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            groups: Vec::with_capacity(0),
        })
    }
}