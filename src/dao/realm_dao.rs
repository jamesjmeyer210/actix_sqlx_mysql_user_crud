use crate::dao::db_context::Table;
use crate::model::Realm;

impl<'c> Table<'c, Realm> {
    pub async fn add_realm(&self, name: &str) -> Result<u64, sqlx::Error> {
        let realm = Realm::from(name);

        sqlx::query(
            r#"
            INSERT INTO `roles` (`name`, `max`)
            VALUES (?, ?)
        "#,
        )
        .bind(realm.name)
        .bind(realm.created_on_utc)
        .execute(&*self.pool)
        .await
        .map(|x|x.rows_affected())
    }

    pub async fn get_realm_by_name(&self, name: &str) -> Result<Realm, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`, `created_on_utc`
            FROM `realms`
            WHERE `name` = ?
        "#,
        )
        .bind(name)
        .fetch_one(&*self.pool)
        .await
    }
}