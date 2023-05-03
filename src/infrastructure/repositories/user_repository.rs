use crate::domain::interfaces::user_repository::IUserRepository;
use crate::domain::models::user::User;
use diesel::prelude::*;

pub struct UserRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> UserRepository<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        UserRepository { conn }
    }
}

impl<'a> IUserRepository for UserRepository<'a> {
    fn find_by_email(&mut self, user_email: &str) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        users.filter(email.eq(user_email)).first(self.conn)
    }

    fn create(&mut self, new_user: &User) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(new_user)
            .get_result(self.conn)
    }
}
