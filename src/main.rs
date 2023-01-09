use std::fs;

use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use jwt_simple::prelude::*;
use sha2::Sha256;
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
        .and(store_filter.clone())
        .and(keypair_filter.clone())
        .and(warp::body::json())
        .and_then(routes::token::print_request);

    let health = warp::get()
        .and(warp::path("_admin"))
        .and(warp::path("health"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::health::health);

    let routes = health.or(token);

    type HmacSha256 = Hmac<Sha256>;

    let salt =
        "pSMrnnFNdJtanr5m+D2ZNHOpszE0sYFTAMWXkLfvR7F0euELeyEu1Q1AqwS7o3RTrHyo0UdYtwexDWe7N3gEyA==";
    let decoded_salt = general_purpose::STANDARD
        .decode(salt)
        .expect("Could note decode salt");
    let hashpassword = "0mhcQnHQjB02bWs9J1u5WFD7e9qZnq32GWfsZjO/XlA=";
    let password = "usr001..";

    let mut mac =
        HmacSha256::new_from_slice(decoded_salt.as_slice()).expect("HMAC can take key of any size");
    mac.update(password.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let encoded: String = general_purpose::STANDARD.encode(code_bytes);
    println!("{}", encoded);
    assert_eq!(encoded, hashpassword);
    println!("Hellorust open for e-business");

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;

    Ok(())
}
