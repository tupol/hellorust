use crate::store::Store;
use crate::types::{logon_request::LogonRequest, token_response::TokenResponse};
use jwt_simple::prelude::*;
use std::fs;
use warp::http::StatusCode;
use warp::reply::with_status;

pub async fn print_request(
    store: Store,
    item: LogonRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", item);
    let private_pem_file_content = fs::read_to_string("authx/privatekey-authx.pkcs8")
        .expect("Should have been able to read the file");
    println!("{}", private_pem_file_content);
    let key_pair =
        RS256KeyPair::from_pem(&private_pem_file_content).expect("Could not read private key");
    let claims = Claims::create(coarsetime::Duration::from_secs(60 * 60 * 2))
        .with_issuer("https://authx.xlinq.io")
        .with_audience("scf.xlinq.io");
    let token = key_pair.sign(claims).expect("Could not sign claims");
    let response = TokenResponse {
        id_token: token,
        access_token: "access".to_string(),
        expires_in: 1000,
        scope: "scope".to_string(),
        token_type: "whatever".to_string(),
    };

    let _res = match store.userInfo().await {
        Ok(p) => println!("Healthy {:?}", p),
        Err(err) => println!("Not healthy {:?}", err),
    };

    Ok(warp::reply::with_status(
        format!("Received logon request for {:?}", response),
        StatusCode::OK,
    ))
}
