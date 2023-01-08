use std::time::Duration;
use warp::Filter;

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = store::Store::new("postgres://maverick:maverick@localhost:5432/datagen").await;
    let store2 = store.clone();
    let store_filter = warp::any().map(move || store.clone());

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(4));
    tokio::spawn(async move {
        loop {
            let store_clone = store2.clone();
            interval.tick().await;
            tokio::spawn(async move {
                match store_clone.ping().await {
                    Ok(_) => println!("Database ping"),
                    Err(err) => println!("Database unhealth {:?}", err),
                }
            });
        }
    });

    // println!("{:?}", store.select().await);

    // POST /token
    let token = warp::post()
        .and(warp::path("token"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(routes::token::print_request);

    let health = warp::get()
        .and(warp::path("_admin"))
        .and(warp::path("health"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::health::health);

    let routes = health.or(token);

    println!("Hellorust open for e-business");

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;

    Ok(())
}
