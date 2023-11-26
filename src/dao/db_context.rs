use super::{Role, User};
use sqlx::{FromRow, SqlitePool};
use std::marker::PhantomData;
use std::sync::Arc;
use sqlx::migrate::MigrateError;
use sqlx::sqlite::SqliteRow;
use crate::model::Realm;

pub struct Table<'c, T>
where
    T: FromRow<'c, SqliteRow>,
{
    pub pool: Arc<SqlitePool>,
    _from_row: fn(&'c SqliteRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, SqliteRow>,
{
    fn new(pool: Arc<SqlitePool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}

pub struct JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, SqliteRow>,
    T2: FromRow<'c, SqliteRow>,
{
    pub pool: Arc<SqlitePool>,
    _from_row: (
        fn(&'c SqliteRow) -> Result<T1, sqlx::Error>,
        fn(&'c SqliteRow) -> Result<T2, sqlx::Error>,
    ),
    _marker_t1: PhantomData<&'c T1>,
    _marker_t2: PhantomData<&'c T2>,
}

impl<'c, T1, T2> JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, SqliteRow>,
    T2: FromRow<'c, SqliteRow>,
{
    fn new(pool: Arc<SqlitePool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
        }
    }
}

pub struct Database<'c> {
    pub realms: Arc<Table<'c, Realm>>,
    pub roles: Arc<Table<'c, Role>>,
    pub users: Arc<Table<'c, User>>,
    pub users_to_groups: Arc<JoinTable<'c, User, Role>>,
    _pool: Arc<SqlitePool>,
}

impl<'a> Database<'a> {
    pub async fn new(sql_url: &str) -> Database<'a> {
        let connection = SqlitePool::connect(&sql_url).await.unwrap();
        let pool = Arc::new(connection);

        Database {
            realms: Arc::from(Table::new(pool.clone())),
            roles: Arc::from(Table::new(pool.clone())),
            users: Arc::from(Table::new(pool.clone())),
            users_to_groups: Arc::from(JoinTable::new(pool.clone())),
            _pool: pool
        }
    }

    pub async fn migrate(&self) -> Result<(), MigrateError>
    {
        sqlx::migrate!().run(&*self._pool).await
    }

    pub async fn initialize(&self) -> Result<(), sqlx::Error> {
        let realm = self.realms.add_default_realm().await?;
        if realm.is_none() {
            return Ok(());
        }

        let realm = realm.unwrap();
        Ok(())
    }
}
