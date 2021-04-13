use super::{Group, User};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool};
use std::sync::Arc;

// pub struct Table<'c, T>
// where
//     T: FromRow<'c, MySqlRow<'c>>,
// {
//     pub pool: Arc<MySqlPool>,
//     _from_row: fn(&MySqlRow<'c>) -> Result<T, sqlx::Error>,
// }

// pub struct Table<'c, T>
//     where
//         T: FromRow<'c, MySqlRow>,
// {
//     pub pool: Arc<MySqlPool>,
//     _from_row: fn(&MySqlRow) -> Result<T, sqlx::Error>,
// }

pub struct Table<T>
    where
        T: FromRow<MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&MySqlRow) -> Result<T, sqlx::Error>,
}

// impl<'c, T> Table<'c, T>
// where
//     T: FromRow<'c, MySqlRow<'c>>,
// {
//     fn new(pool: Arc<MySqlPool>) -> Self {
//         Table {
//             pool,
//             _from_row: T::from_row,
//         }
//     }
// }

// impl<'c, T> Table<'c, T>
//     where
//         T: FromRow<'c, MySqlRow>,
// {
//     fn new(pool: Arc<MySqlPool>) -> Self {
//         Table {
//             pool,
//             _from_row: T::from_row,
//         }
//     }
// }

impl<T> Table<T>
    where
        T: FromRow<MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
        }
    }
}

// pub struct JoinTable<'c, T1, T2>
// where
//     T1: FromRow<'c, MySqlRow<'c>>,
//     T2: FromRow<'c, MySqlRow<'c>>,
// {
//     pub pool: Arc<MySqlPool>,
//     _from_row: (
//         fn(&MySqlRow<'c>) -> Result<T1, sqlx::Error>,
//         fn(&MySqlRow<'c>) -> Result<T2, sqlx::Error>,
//     ),
// }

// pub struct JoinTable<'c, T1, T2>
//     where
//         T1: FromRow<'c, MySqlRow>,
//         T2: FromRow<'c, MySqlRow>,
// {
//     pub pool: Arc<MySqlPool>,
//     _from_row: (
//         fn(&MySqlRow) -> Result<T1, sqlx::Error>,
//         fn(&MySqlRow) -> Result<T2, sqlx::Error>,
//     ),
// }

pub struct JoinTable<T1, T2>
    where
        T1: FromRow<MySqlRow>,
        T2: FromRow<MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: (
        fn(&MySqlRow) -> Result<T1, sqlx::Error>,
        fn(&MySqlRow) -> Result<T2, sqlx::Error>,
    ),
}

// impl<'c, T1, T2> JoinTable<'c, T1, T2>
// where
//     T1: FromRow<'c, MySqlRow<'c>>,
//     T2: FromRow<'c, MySqlRow<'c>>,
// {
//     fn new(pool: Arc<MySqlPool>) -> Self {
//         JoinTable {
//             pool,
//             _from_row: (T1::from_row, T2::from_row),
//         }
//     }
// }

// impl<'c, T1, T2> JoinTable<'c, T1, T2>
//     where
//         T1: FromRow<'c, MySqlRow>,
//         T2: FromRow<'c, MySqlRow>,
// {
//     fn new(pool: Arc<MySqlPool>) -> Self {
//         JoinTable {
//             pool,
//             _from_row: (T1::from_row, T2::from_row),
//         }
//     }
// }

impl<T1, T2> JoinTable<T1, T2>
    where
        T1: FromRow<MySqlRow>,
        T2: FromRow<MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
        }
    }
}

// pub struct Database<'c> {
//     pub users: Arc<Table<'c, User>>,
//     pub groups: Arc<Table<'c, Group>>,
//     pub users_to_groups: Arc<JoinTable<'c, User, Group>>,
// }

pub struct Database {
    pub users: Arc<Table<User>>,
    pub groups: Arc<Table<Group>>,
    pub users_to_groups: Arc<JoinTable<User, Group>>,
}

impl Database {
    pub async fn new(sql_url: &str) -> Database {
        let pool = MySqlPool::new(sql_url).await.unwrap();
        let pool = Arc::new(pool);

        Database {
            users: Arc::from(Table::new(pool.clone())),
            groups: Arc::from(Table::new(pool.clone())),
            users_to_groups: Arc::from(JoinTable::new(pool.clone())),
        }
    }
}
