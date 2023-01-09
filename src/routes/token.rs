use jwt_simple::prelude::*;
use warp::http::StatusCode;
use warp::reply::with_status;

use crate::store::Store;
use crate::types::{logon_request::LogonRequest, token_response::TokenResponse};

pub async fn print_request(
    store: Store,
    keypair: RS256KeyPair,
    item: LogonRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", item);

    let users = store.user_info(&item.username).await.unwrap();
    if users.len() != 1 {
        Ok(with_status(
            format!("Zero or more users found {:?}", users),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        let claims = Claims::create(coarsetime::Duration::from_secs(60 * 60 * 2))
            .with_issuer("https://authx.xlinq.io")
            .with_audience("scf.xlinq.io");
        let token = keypair.sign(claims).expect("Could not sign claims");
        let response = TokenResponse {
            id_token: token,
            access_token: "access".to_string(),
            expires_in: 1000,
            scope: "scope".to_string(),
            token_type: "whatever".to_string(),
        };

        Ok(warp::reply::with_status(
            format!("Received logon request for {:?}", response),
            StatusCode::OK,
        ))
    }
}
