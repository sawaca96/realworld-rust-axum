use crate::domain::interfaces::user_repository::IUserRepository;
use crate::domain::models::user::{Me, User};
use crate::infrastructure::database::db_connection::PgConnection;
use crate::schema::users;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub struct UserRepository {
    conn: PgConnection,
}

impl UserRepository {
    pub fn new(conn: PgConnection) -> Self {
        UserRepository { conn }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn get_user_by_email(&mut self, email: &str) -> QueryResult<User> {
        users::table
            .filter(users::email.eq(email))
            .get_result(&mut self.conn)
            .await
    }

    async fn create(&mut self, user: &User) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(user)
            .get_result(&mut self.conn)
            .await
    }

    async fn get_me(&mut self, id: &uuid::Uuid) -> QueryResult<Me> {
        let value = users::table
            .filter(users::id.eq(id))
            .get_result(&mut self.conn)
            .await;
        value.map(|user: User| Me {
            id: user.id,
            email: user.email,
            nickname: user.nickname,
        })
    }
}
