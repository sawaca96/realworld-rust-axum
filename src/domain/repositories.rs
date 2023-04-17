use crate::domain::models::user::User;
use uuid::Uuid;

// src/domain/repositories.rs

pub trait UserRepository {
    fn create(&mut self, user: &User) -> Result<User, diesel::result::Error>;
    fn find_by_id(&mut self, id: Uuid) -> Result<Option<User>, diesel::result::Error>;
    fn update(&mut self, user: &User) -> Result<User, diesel::result::Error>;
    fn delete(&mut self, id: Uuid) -> Result<usize, diesel::result::Error>;
}
