use super::{init_db_context, randomize_string};
use sqlx;
use sqlx_user_crud::model::User;
use uuid::Uuid;

#[actix_rt::test]
async fn add_user_returns_1() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("alice"),
        email: "alice@email.com".as_bytes().to_vec(),
        email_verified: false,
        phone: "111-222-3322".as_bytes().to_vec(),
        phone_verified: false,
        public_key: Vec::new(),
        groups: Vec::new(),
    };

    let result = db.users.add_user(&user).await;

    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());

    Ok(())
}

#[actix_rt::test]
async fn add_user_returns_err_when_duplicate_username_is_added(
) -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let name = randomize_string("bob");
    let email = randomize_string("bob@emai.com");

    let original = User::new("bob", "bob@email.com", "111-222-3344");
    let duplicate = User::new("bob", "bob@email.com", "111-222-3344");

    let result = db.users.add_user(&original).await?;
    assert_eq!(1, result);

    let result = db.users.add_user(&duplicate).await;
    assert!(result.is_err());

    Ok(())
}

#[actix_rt::test]
async fn get_user_by_id_returns_error_when_user_does_not_exist() -> () {
    let db = init_db_context().await;

    let id = Uuid::new_v4().to_string();

    let result = db.users.get_user_by_id(&id).await;
    assert!(result.is_err());
}

#[actix_rt::test]
async fn get_user_by_id_returns_user_when_user_exists() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let user = User::new("charlie", "charlie@email.com", "111-222-3344");

    let _ = db.users.add_user(&user).await?;

    let result = db.users.get_user_by_id(&user.id).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(user.name, result.name);
    assert_eq!(user.email, result.email);
    Ok(())
}

#[actix_rt::test]
async fn update_user_returns_zero_when_user_does_not_exist() -> () {
    let db = init_db_context().await;

    let user = User::new("david", "david@email.com", "111-222-3344");

    let result = db.users.update_user(&user).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result);
}

#[actix_rt::test]
async fn update_user_returns_1_when_user_exists() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let user = User::new("emily", "emily@email.com", "111-222-3344");

    let _ = db.users.add_user(&user).await?;

    let mut updated_user = user.clone();
    updated_user.email = "emily_edison@email.com".as_bytes().to_vec();

    let result = db.users.update_user(&updated_user).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result);
    Ok(())
}

#[actix_rt::test]
async fn delete_user_returns_0_when_user_does_not_exist() -> () {
    let db = init_db_context().await;
    let id = Uuid::new_v4().to_string();

    let result = db.users.delete_user(&id).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result);
}

#[actix_rt::test]
async fn delete_user_returns_1_when_user_exists() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let user = User::new("gary", "gary@email.com", "111-222-3344");

    let _ = db.users.add_user(&user).await?;

    let result = db.users.delete_user(&user.id).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result);
    Ok(())
}
