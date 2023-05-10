use crate::domain::models::user::{Me, User};
use async_trait::async_trait;
use diesel::result::QueryResult;

#[async_trait]
pub trait IUserRepository {
    async fn get_user_by_email(&mut self, email: &str) -> QueryResult<User>;
    async fn create(&mut self, new_user: &User) -> QueryResult<User>;
    async fn get_me(&mut self, id: &uuid::Uuid) -> QueryResult<Me>;
}
