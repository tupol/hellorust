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
    //username text, hashpassword text, salt text, loa1level integer, loa2level integer, amid text,
    // amstate character varying, amlocktime
    // integer, name text, emailaddress text, typeuser text, firstname text, lastname text,
    // usertechnicalid text, pwcreated date)
    pub async fn userInfo(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query("SELECT * FROM userinfo($1, $2, $3);")
            .bind("NPA-AuthenticationManagement")
            .bind("confidential")
            .bind("confidential")
            .map(|row: PgRow| User {
                username: row.get("username"),
                hashpassword: row.get("hashpassword"),
                salt: row.get("salt"),
                loa1level: row.get("loa1level"),
                loa2level: row.get("loa2level"),
                amid: row.get("amid"),
                amstate: row.get("amstate"),
                amlocktime: row.get("amlocktime"),
                name: row.get("name"),
                emailaddress: row.get("emailaddress"),
                typeuser: row.get("typeuser"),
                firstname: row.get("firstname"),
                lastname: row.get("lastname"),
                usertechnicalid: row.get("usertechnicalid"),
                pwcreated: row.get("pwcreated"),
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
