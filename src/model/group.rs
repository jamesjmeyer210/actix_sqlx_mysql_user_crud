use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row, Error};

// TODO: Try to use the #[derive(sqlx::FromRow)] here.
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Group {
    pub id: u64,
    pub name: String,
}

// impl<'r> FromRow<'r, MySqlRow> for Group {
//     fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
//         Ok(Group {
//             id: row.get(0),
//             name: row.get(1),
//         })
//     }
// }

impl FromRow<'_, MySqlRow> for Group {
    fn from_row(row: &MySqlRow) -> Result<Self, Error> {
        Ok(Group {
            id: row.get(0),
            name: row.get(1),
        })
    }
}