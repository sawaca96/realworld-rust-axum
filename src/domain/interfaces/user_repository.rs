use crate::domain::models::user::User;
use async_trait::async_trait;
use diesel::result::QueryResult;

#[async_trait]
pub trait IUserRepository {
    async fn find_by_email(&mut self, email: &str) -> QueryResult<User>; // TODO: User가 아닌 Me반환 필요
    async fn create(&mut self, new_user: &User) -> QueryResult<User>;
}
