use crate::domain::interfaces::user_repository::UserRepository;
use crate::domain::models::user::User;
use crate::infrastructure::database::db_connection::PgPooledConnection;
use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

pub struct UserRepositoryImpl {
    conn: PgPooledConnection,
}

impl UserRepositoryImpl {
    pub fn new(conn: PgPooledConnection) -> Self {
        UserRepositoryImpl { conn }
    }

    // Helper function to get a mutable reference to the connection
    fn conn_mut(&mut self) -> &mut PgConnection {
        &mut self.conn
    }
}

impl UserRepository for UserRepositoryImpl {
    fn create(&mut self, user: &User) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(user)
            .get_result(self.conn_mut())
    }
    fn find_by_id(&mut self, id: Uuid) -> Result<Option<User>, diesel::result::Error> {
        users::table
            .filter(users::id.eq(id))
            .first(self.conn_mut())
            .optional()
    }
    fn update(&mut self, user: &User) -> Result<User, diesel::result::Error> {
        diesel::update(users::table.filter(users::id.eq(user.id)))
            .set(user)
            .get_result(self.conn_mut())
    }

    fn delete(&mut self, id: Uuid) -> Result<usize, diesel::result::Error> {
        diesel::delete(users::table.filter(users::id.eq(id))).execute(self.conn_mut())
    }
}
