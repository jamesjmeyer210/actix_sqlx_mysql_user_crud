use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;

pub struct Count(i32);

impl Count {
    pub fn into_inner(self) -> i32 {
        self.0
    }
}

impl<'r> FromRow<'r, SqliteRow> for Count {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        row.try_get(0).map(|x|Count(x))
    }
}