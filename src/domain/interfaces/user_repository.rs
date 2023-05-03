use crate::domain::models::user::User;
use diesel::result::QueryResult;

pub trait IUserRepository {
    fn find_by_email(&mut self, email: &str) -> QueryResult<User>;
    fn create(&mut self, new_user: &User) -> QueryResult<User>;
}
