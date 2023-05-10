use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

pub type PgPool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type PgConnection =
    bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn create_pg_pool(
    db_url: &str,
) -> bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();
    pool
}
