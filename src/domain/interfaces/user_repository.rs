use crate::domain::models::user::User;
use diesel::result::QueryResult;

pub trait IUserRepository {
    fn find_by_email(&mut self, email: &str) -> QueryResult<User>; // TODO: User가 아닌 Me반환 필요
    fn create(&mut self, new_user: &User) -> QueryResult<User>;
}
