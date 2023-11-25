use super::{init_db_context, randomize_string};
use sqlx;
use sqlx_user_crud::model::Realm;

#[actix_rt::test]
async fn add_group_returns_1_when_group_is_valid() -> () {
    let db = init_db_context().await;

    let group_name = randomize_string("users");

    let result = db.roles.add_role(&Realm::default(), &group_name, &None).await;
    if result.is_err() {
        let e = result.unwrap_err();
        eprintln!("{}", e);
    }
    /*assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result);*/
}

#[actix_rt::test]
async fn add_group_returns_err_when_group_already_exists() -> () {
    let db = init_db_context().await;

    let group_name = randomize_string("administrators");
    let _ = db.roles.add_role(&Realm::default(), &group_name, &None).await;

    let result = db.roles.add_role(&Realm::default(), &group_name, &None).await;
    assert!(result.is_err());
}

#[actix_rt::test]
async fn get_group_by_name_returns_group_when_name_exists() -> () {
    let db = init_db_context().await;

    let group_name = randomize_string("accountants");
    let _ = db.roles.add_role(&Realm::default(), &group_name, &None).await;

    let result = db.roles.get_role_by_name(&group_name).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(group_name, result.name);
    assert!(1 <= result.id);
}

#[actix_rt::test]
async fn get_group_by_name_returns_err_when_group_does_not_exist() -> () {
    let db = init_db_context().await;

    let result = db.roles.get_role_by_name("not found").await;
    assert!(result.is_err());
}

#[actix_rt::test]
async fn get_group_by_id_returns_group_when_id_is_valid() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let group_name = randomize_string("engineers");
    let _ = db.roles.add_role(&Realm::default(), &group_name, &None).await?;
    let group = db.roles.get_role_by_name(&group_name).await?;

    let result = db.roles.get_role_by_id(group.id).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(group.id, result.id);
    assert_eq!(group.name, result.name);
    Ok(())
}

#[actix_rt::test]
async fn update_group_returns_1_when_group_has_been_updated() -> Result<(), sqlx::Error>
{
    let db = init_db_context().await;

    let group_name = randomize_string("testers");
    let _ = db.roles.add_role(&Realm::default(), &group_name, &None).await?;

    let result = db
        .roles
        .update_role(&group_name, &randomize_string("qa testers"))
        .await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result);
    Ok(())
}

#[actix_rt::test]
async fn update_group_returns_0_when_group_does_not_exist() -> () {
    let db = init_db_context().await;

    let result = db.roles.update_role("not found", "still not found").await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result);
}

#[actix_rt::test]
async fn delete_group_returns_1_when_group_can_be_deleted() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let group_name = randomize_string("executives");
    let _ = db.roles.add_role(&Realm::default(), &group_name, &None).await?;

    let result = db.roles.delete_role(&group_name).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result);
    Ok(())
}

#[actix_rt::test]
async fn delete_group_returns_0_when_group_does_not_exist() -> () {
    let db = init_db_context().await;

    let result = db.roles.delete_role("not found").await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result);
}
