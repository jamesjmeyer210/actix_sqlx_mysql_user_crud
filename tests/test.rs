use sqlx_user_crud::dao::Database;
use uuid::Uuid;

fn randomize_string(input: &'static str) -> String {
    format!("{0}{1}", input, Uuid::new_v4().to_string())
}

async fn init_db_context() -> Database<'static> {
    let db = Database::new("sqlite::memory:").await;
    let x = db.migrate().await;
    if x.is_err() {
        panic!("{}", x.unwrap_err().to_string())
    }
    db
}

#[cfg(test)]
mod test {
    use super::init_db_context;

    #[actix_rt::test]
    async fn in_memory_db_connects()
    {
        let db = init_db_context().await;
        let x = db.migrate().await;
        assert!(x.is_ok())
    }

}

#[cfg(test)]
mod controller_test;

#[cfg(test)]
mod dao_test;