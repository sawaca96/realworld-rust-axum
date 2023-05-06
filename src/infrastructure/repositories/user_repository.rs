use std::borrow::BorrowMut;

use crate::domain::interfaces::user_repository::IUserRepository;
use crate::domain::models::user::User;
use crate::infrastructure::database::db_connection::PgPooledConnection;
use diesel::prelude::*;

pub struct UserRepository {
    conn: PgPooledConnection,
}

impl UserRepository {
    pub fn new(conn: PgPooledConnection) -> Self {
        UserRepository { conn }
    }
}

impl IUserRepository for UserRepository {
    fn find_by_email(&mut self, user_email: &str) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        users
            .filter(email.eq(user_email))
            .first(self.conn.borrow_mut())
    }

    fn create(&mut self, new_user: &User) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(new_user)
            .get_result(self.conn.borrow_mut())
    }
}
