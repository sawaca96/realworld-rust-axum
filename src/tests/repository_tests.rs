use crate::domain::interfaces::user_repository::UserRepository;
use crate::domain::models::user::User;
use crate::infrastructure::repositories::user_repository::UserRepositoryImpl;
use crate::test_utils::get_test_db_connection;
use uuid::Uuid;

#[test]
fn test_user_repository_crud() {
    let conn = get_test_db_connection();
    let mut user_repository = UserRepositoryImpl::new(conn);

    let user = User {
        id: Uuid::new_v4(),
        nickname: "TestUser".to_string(),
        email: "test@example.com".to_string(),
    };

    // Test create
    let created_user = user_repository.create(&user).unwrap();
    assert_eq!(user, created_user);

    // Test find_by_id
    let found_user = user_repository.find_by_id(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);

    // Test update
    let updated_user = User {
        id: user.id,
        nickname: "UpdatedUser".to_string(),
        email: "updated@example.com".to_string(),
    };
    let saved_updated_user = user_repository.update(&updated_user).unwrap();
    assert_eq!(updated_user, saved_updated_user);

    // Test delete
    let deleted_count = user_repository.delete(user.id).unwrap();
    assert_eq!(1, deleted_count);

    // Test find_by_id after delete
    let not_found_user = user_repository.find_by_id(user.id).unwrap();
    assert!(not_found_user.is_none());
}
