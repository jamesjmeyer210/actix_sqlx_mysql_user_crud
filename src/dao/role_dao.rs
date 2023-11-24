use super::Role;
use super::Table;

impl<'c> Table<'c, Role> {
    pub async fn get_role_by_id(&self, id: i32) -> Result<Role, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `max`, `name`
            FROM `roles`
            WHERE `id` = ?
        "#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn get_role_by_name(&self, name: &str) -> Result<Role, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `max`, `name`
            FROM `roles`
            WHERE `name` = ?
        "#,
        )
        .bind(name)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn add_role(&self, name: &str, max: &Option<i32>) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO `roles` (`name`, `max`)
            VALUES (?, ?)
        "#,
        )
        .bind(name)
        .bind(max)
        .execute(&*self.pool)
        .await
        .map(|x|x.rows_affected())
    }

    pub async fn update_role(&self, current: &str, update: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE `roles`
            SET `name` = ?
            WHERE `name` = ?
        "#,
        )
        .bind(update)
        .bind(current)
        .execute(&*self.pool)
        .await
        .map(|x|x.rows_affected())
    }

    pub async fn delete_role(&self, name: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM `roles`
            WHERE `name` = ?
        "#,
        )
        .bind(name)
        .execute(&*self.pool)
        .await
        .map(|x|x.rows_affected())
    }
}
