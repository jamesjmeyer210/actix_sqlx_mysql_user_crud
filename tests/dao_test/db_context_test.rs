use sqlx_user_crud::dao::Database;

#[actix_rt::test]
async fn new_returns_db_context_when_url_is_valid() {
    let db_context = Database::new("sqlite::memory:").await;
    let x = db_context.migrate().await;

    assert!(x.is_ok());
}
