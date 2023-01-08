use std::time::Duration;

use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;
use warp::{http::StatusCode, Filter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug, Serialize, Deserialize)]
    struct LogonRequest {
        username: String,
        password: String,
    }

    #[derive(Debug)]
    struct User {
        id: Uuid,
        name: String,
        email: String,
        password: String,
    }

    async fn print_request(item: LogonRequest) -> Result<impl warp::Reply, warp::Rejection> {
        println!("{:?}", item);
        Ok(warp::reply::with_status(
            format!("Received logon request for {:?}", item.username),
            StatusCode::OK,
        ))
    }

    let mut sched = JobScheduler::new()
        .await
        .expect("Scheduler starting problem");

    let job = Job::new_repeated(Duration::from_secs(7), |_uuid, _l| {
        println!("I'm repeated every 7 seconds");
    })
    .unwrap();

    sched.add(job).await.expect("Could not add job");

    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Scheduler shutting down done");
        })
    }));
    sched.start().await.expect("`Could not start scheduler");

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
                Err(e) => panic!("Couldn't establish DB connection: {}", e),
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
    }

    let store = Store::new("postgres://maverick:maverick@localhost:5432/datagen").await;

    println!("{:?}", store.select().await);

    // let store_filter = warp::any().map(move || store.clone());

    // POST /token
    let token = warp::post()
        .and(warp::path("token"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(print_request);

    let routes = token;

    println!("Hellorust open for e-business");

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;

    Ok(())
}
