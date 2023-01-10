use std::fs;

use jwt_simple::prelude::*;

use warp::Filter;

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = store::Store::new("postgres://maverick:maverick@localhost:5432/datagen").await;
    let store2 = store.clone();
    let store_filter = warp::any().map(move || store.clone());

    let private_pem_file_content = fs::read_to_string("authx/privatekey-authx.pkcs8")
        .expect("Should have been able to read the file");
    // println!("{}", private_pem_file_content);
    let key_pair =
        RS256KeyPair::from_pem(&private_pem_file_content).expect("Could not read private key");
    let keypair_filter = warp::any().map(move || key_pair.clone());

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
        .and(store_filter.clone())
        .and(keypair_filter.clone())
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
