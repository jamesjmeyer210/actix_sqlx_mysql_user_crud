use super::Table;
use super::User;

impl<'c> Table<'c, User> {
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`, `email`
            FROM `users`
            WHERE `id` = ?"#,
        )
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn add_user(&self, user: &User) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (`id`, `name`, `password`, `email`, `email_verified`, `phone`, `phone_verified`, `public_key`, `created_on_utc`, `deleted_on_utc`)
            VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
            .bind(&user.id)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.email_verified)
            .bind(&user.phone)
            .bind(&user.phone_verified)
            .bind(&user.public_key)
            .bind(&user.created_on_utc)
            .bind(&user.deleted_on_utc)
            .execute(&*self.pool)
            .await
            .map(|x|x.rows_affected())
    }

    pub async fn update_user(&self, user: &User) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users
            SET `name` = ?, `email` = ?
            WHERE `id` = ?
            "#,
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.id)
        .execute(&*self.pool)
        .await
        .map(|x|x.rows_affected())
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE `id` = ?
            "#,
        )
        .bind(user_id)
        .execute(&*self.pool)
        .await
        .map(|x|x.rows_affected())
    }
}
