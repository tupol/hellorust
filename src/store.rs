use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::types::user::User;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}
impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {:?}", e),
        };
        Store {
            connection: db_pool,
        }
    }
    pub async fn select(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query("SELECT * FROM users")
            .map(|row: PgRow| User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_all(&self.connection)
            .await
    }
    pub async fn ping(&self) -> Result<i32, sqlx::Error> {
        sqlx::query("SELECT 1")
            .map(|row: PgRow| row.get(0))
            .fetch_one(&self.connection)
            .await
    }
}
