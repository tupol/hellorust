mod auth;

mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;
    use crate::auth::claims::IdClaims;

    #[derive(Debug, Serialize)]
    pub struct TokenResponse {
        pub access_token: String,
        pub id_token: String,
        pub scope: String,
        pub expires_in: i32,
        pub token_type: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LogonRequest {
        pub username: String,
        pub password: String,
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "users")] // singular 'user' is a keyword..
    pub struct User {
        pub username: String,
        pub hashpassword: String,
        pub salt: String,
        pub loa1level: i32,
        pub loa2level: i32,
        pub amid: String,
        pub amstate: String,
        pub amlocktime: i32,
        pub name: String,
        pub emailaddress: String,
        pub typeuser: String,
        pub firstname: String,
        pub lastname: String,
        pub usertechnicalid: String,
        // pub pwcreated: chrono::NaiveDate,
    }
    impl User {
        pub fn to_id_claims(self) -> IdClaims {
            IdClaims {
                name: self.name,
                email: self.emailaddress,
                first_name: self.firstname,
                last_name: self.lastname,
                at_hash: None
            }
        }
    }
}

mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::{errors::MyError, models::{User, LogonRequest}};

    pub async fn get_user(client: &Client, user_info: LogonRequest) -> Result<User, MyError> {
        let _stmt = include_str!("../sql/select_user.sql");
        // let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
            .query(
                &stmt,
                &[
                    &user_info.username,
                    &"confidential".to_string(),
                    &"confidential".to_string(),
                ],
            )
            .await?
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop()
            .ok_or(MyError::NotFound) // more applicable for SELECTs
    }
}

mod handlers {
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use chrono::{prelude::*, Duration};
    use base64::{engine::general_purpose, Engine as _};
    use crate::{db, errors::MyError, models::{LogonRequest, TokenResponse}};
    use std::fs;
    use std::time::Instant;
    use jsonwebtoken::{encode, Algorithm, EncodingKey, DecodingKey, Header};
    use crate::auth::claims::{AccessClaims, JwtClaim};
    use crate::auth::tokens::TokenPair;

    pub async fn add_user(
        logon_req: web::Json<LogonRequest>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let start = Instant::now();
        println!("Logon endpoint invoked");
        let user_info: LogonRequest = logon_req.into_inner();
        let pw = user_info.password.clone();
        let m1 = start.elapsed();

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
        let new_user = db::get_user(&client, user_info).await?;
        let m2 = start.elapsed();

        // check password
        type HmacSha256 = Hmac<Sha256>;
        let salt = new_user.salt.to_string();
        // "pSMrnnFNdJtanr5m+D2ZNHOpszE0sYFTAMWXkLfvR7F0euELeyEu1Q1AqwS7o3RTrHyo0UdYtwexDWe7N3gEyA==";
        let decoded_salt = general_purpose::STANDARD
            .decode(salt)
            .expect("Could note decode salt");
        let hashpassword = new_user.hashpassword.to_string();
        //"0mhcQnHQjB02bWs9J1u5WFD7e9qZnq32GWfsZjO/XlA=";
        let password = pw;

        let mut mac = HmacSha256::new_from_slice(decoded_salt.as_slice())
            .expect("HMAC can take key of any size");
        mac.update(password.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let encoded: String = general_purpose::STANDARD.encode(code_bytes);

        if encoded != hashpassword {
            println!("Incorrect password for user ");
            Ok(HttpResponse::InternalServerError().body("error"))
        } else {
            println!("Logon success for user");
            let n1 = start.elapsed();

            let encoding_key = EncodingKey::from_rsa_pem(include_bytes!("../privatekey.pkcs8"))
                .expect("Should have been able to read the file");
            // let decoding_key = DecodingKey::from_rsa_pem(include_bytes!("../publickey.pem"))
            //     .expect("Should have been able to read the file");

            let n2 = start.elapsed();

            let header = Header::new(Algorithm::RS256);

            let common_claims = JwtClaim::empty()
                .with_audience("scf.xlinq.io".to_string())
                .with_subject("NPA-PlatformManagement".to_string())
                .with_issuer("https://authx.xlinq.io".to_string())
                .issued_now()
                .expires_in(Duration::minutes(60).num_seconds().unsigned_abs());

            let id_claims = new_user.to_id_claims();
            let access_claims = AccessClaims{ session_id: "session_id".to_string() };

            let token_pair = TokenPair::create(&encoding_key, &header, common_claims, id_claims, access_claims).unwrap();
            let id_token = token_pair.id_token.raw_token(&encoding_key).unwrap();
            let access_token = token_pair.access_token.raw_token(&encoding_key).unwrap();

            let n3 = start.elapsed();
            // let token = key_pair.sign(claims).expect("Could not sign claims");
            let response = TokenResponse {
                id_token: id_token,
                access_token: access_token,
                expires_in: 1000,
                scope: "scope".to_string(),
                token_type: "whatever".to_string(),
            };

            let m3 = start.elapsed();
            println!("Elapsed total {} database {} file {} sign {}", m3.as_millis(), (m2-m1).as_millis(),
                     (n2-n1).as_millis(), (n3-n2).as_millis());
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::add_user;
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;
use crate::handlers::TokenHandler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let handler = TokenHandler::create();

    // pub async fn add_user(
    //     logon_req: web::Json<crate::models::LogonRequest>,
    //     db_pool: web::Data<deadpool_postgres::Pool>,
    // ) -> Result<actix_web::HttpResponse, actix_web::Error> {
    //     handler.
    // }

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/token").route(web::post().to(add_user )))
    })
        // .workers(50)
        .bind(config.server_addr.clone())?
        .run();
    println!("Server running at http://{}/", config.server_addr);


    server.await
}