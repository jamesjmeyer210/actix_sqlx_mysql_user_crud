use super::{init_db_context, randomize_string};
use sqlx;
use sqlx_user_crud::model::{Realm, Role, User};
use uuid::Uuid;

#[actix_rt::test]
async fn add_user_groups_returns_1_when_user_is_associated_with_group(
) -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let user = User::new("alice", "alice@email.com", "111-222-3344");

    let group = randomize_string("user");

    let _ = db.users.add_user(&user).await?;
    let _ = db.roles.add_role(&Realm::default(), &group, &None).await?;

    let group = db.roles.get_role_by_name(&group).await?;
    let groups = vec![group];

    let result = db.users_to_groups.add_user_groups(&user.id, &groups).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result);
    Ok(())
}

#[actix_rt::test]
async fn add_user_groups_returns_3_when_user_is_associated_with_3_groups(
) -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let user = User::new("bob", "bob@email.com", "111-222-3344");

    let group_names = vec![
        randomize_string("engineer"),
        randomize_string("architect"),
        randomize_string("tester"),
    ];

    let _ = db.users.add_user(&user).await?;
    for group_name in group_names.iter() {
        let _ = db.roles.add_role(&Realm::default(), group_name, &None).await?;
    }

    let mut groups = Vec::with_capacity(3);
    for group_name in group_names.iter() {
        let group = db.roles.get_role_by_name(group_name).await?;
        groups.push(group);
    }

    let result = db.users_to_groups.add_user_groups(&user.id, &groups).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(3, result);
    Ok(())
}

#[actix_rt::test]
async fn add_user_groups_returns_err_when_group_does_not_exist(
) -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let user = User::new("charlie", "charlie@email.com", "111-222-3344");
    let groups = vec![Role {
        id: 0,
        max: None,
        name: String::from("non-existent"),
    }];

    let _ = db.users.add_user(&user).await?;

    let result = db.users_to_groups.add_user_groups(&user.id, &groups).await;
    assert!(result.is_err());
    Ok(())
}

#[actix_rt::test]
async fn add_user_groups_returns_err_when_user_does_not_exist() -> Result<(), sqlx::Error>
{
    let db = init_db_context().await;

    let group_name = randomize_string("hackers");
    let _ = db.roles.add_role(&Realm::default(), &group_name, &None).await?;
    let group = db.roles.get_role_by_name(&group_name).await?;
    let groups = vec![group];

    let result = db
        .users_to_groups
        .add_user_groups(&Uuid::new_v4().to_string(), &groups)
        .await;
    assert!(result.is_err());
    Ok(())
}

#[actix_rt::test]
async fn get_groups_by_user_id_returns_users_groups() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    let user = User::new("candice", "candice@email.com", "111-222-3344");
    let group = randomize_string("vendor");

    {
        let _ = db.users.add_user(&user).await?;
        let _ = db.roles.add_role(&Realm::default(), &group, &None).await?;
        let group = db.roles.get_role_by_name(&group).await?;
        let groups = vec![group];
        let _ = db
            .users_to_groups
            .add_user_groups(&user.id, &groups)
            .await?;
    }

    let result = db.users_to_groups.get_groups_by_user_id(&user.id).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result.len());
    assert_eq!(group, result[0].name);
    Ok(())
}

#[actix_rt::test]
async fn get_groups_by_user_id_returns_empty_vec_when_user_does_not_exist() -> () {
    let db = init_db_context().await;
    let user_id = Uuid::new_v4().to_string();

    let result = db.users_to_groups.get_groups_by_user_id(&user_id).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result.len());
}

#[actix_rt::test]
async fn delete_by_user_id_returns_0_when_user_id_does_not_exist() -> () {
    let db = init_db_context().await;
    let user_id = Uuid::new_v4().to_string();

    let result = db.users_to_groups.delete_by_user_id(&user_id).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result);
}

#[actix_rt::test]
async fn delete_by_user_id_returns_number_of_rows_deleted() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    let user = User::new("donald", "donald@email.com", "111-222-3344");
    let group = randomize_string("customer");

    {
        let _ = db.users.add_user(&user).await?;
        let _ = db.roles.add_role(&Realm::default(), &group, &None).await?;
        let group = db.roles.get_role_by_name(&group).await?;
        let groups = vec![group];
        let _ = db
            .users_to_groups
            .add_user_groups(&user.id, &groups)
            .await?;
    }

    let result = db.users_to_groups.delete_by_user_id(&user.id).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result);
    Ok(())
}

#[actix_rt::test]
async fn delete_by_group_id_returns_0_when_group_id_does_not_exist() -> () {
    let db = init_db_context().await;

    let result = db.users_to_groups.delete_by_group_id(0).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result);
}

#[actix_rt::test]
async fn delete_by_group_id_returns_number_of_rows_deleted() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    let user = User::new("emily", "emily@email.com", "111-222-3344");
    let group = randomize_string("executive");

    {
        let _ = db.users.add_user(&user).await?;
        let _ = db.roles.add_role(&Realm::default(), &group, &None).await?;
    }

    let group = db.roles.get_role_by_name(&group).await?;
    {
        let groups = vec![group.clone()];
        let _ = db
            .users_to_groups
            .add_user_groups(&user.id, &groups)
            .await?;
    }

    let result = db.users_to_groups.delete_by_group_id(group.id).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result);
    Ok(())
}

#[actix_rt::test]
async fn update_user_groups_deletes_rows_when_users_group_vec_is_empty(
) -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    let user = User::new("frank", "frank@email.com", "111-222-3344");

    let group_name = randomize_string("faculty");
    {
        let _ = db.users.add_user(&user).await?;
        let _ = db.roles.add_role(&Realm::default(), &group_name, &None).await?;
    }

    let group = db.roles.get_role_by_name(&group_name).await?;
    {
        let groups = vec![group.clone()];
        let _ = db
            .users_to_groups
            .add_user_groups(&user.id, &groups)
            .await?;
    }
    // Assert the function returns 1 modification
    let result = db.users_to_groups.update_user_groups(&user).await;
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());
    // Verify the function has altered the table
    let result = db.users_to_groups.get_groups_by_user_id(&user.id).await?;
    assert_eq!(0, result.len());
    Ok(())
}

#[actix_rt::test]
async fn update_user_groups_returns_deleted_plus_added_rows_when_groups_is_not_empty(
) -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    let mut user = User::new("frank", "frank@email.com", "111-222-3344");

    let group_names = vec![randomize_string("general"), randomize_string("gossiper")];

    {
        let _ = db.users.add_user(&user).await?;
        let _ = db.roles.add_role(&Realm::default(), &group_names[0], &None).await?;
        let _ = db.roles.add_role(&Realm::default(), &group_names[1], &None).await?;
    }

    let groups = vec![
        db.roles.get_role_by_name(&group_names[0]).await?,
        db.roles.get_role_by_name(&group_names[1]).await?,
    ];
    {
        let groups = vec![groups[0].clone()];
        let _ = db
            .users_to_groups
            .add_user_groups(&user.id, &groups)
            .await?;
    }

    user.groups = groups;
    // Assert the function returns 1 modification
    let result = db.users_to_groups.update_user_groups(&user).await;
    assert!(result.is_ok());
    assert_eq!(3, result.unwrap());
    Ok(())
}
