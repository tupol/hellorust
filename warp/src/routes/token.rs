use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use jwt_simple::prelude::*;
use sha2::Sha256;
use warp::http::StatusCode;
use warp::reply::with_status;

use crate::store::Store;
use crate::types::{logon_request::LogonRequest, token_response::TokenResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdClaims {
    username: String,
    email: String,
}

pub async fn print_request(
    item: LogonRequest,
    store: Store,
    keypair: RS256KeyPair,
) -> Result<impl warp::Reply, warp::Rejection> {
    // println!("{:?}", item);

    let users = store.user_info(&item.username).await.unwrap();
    if users.len() != 1 {
        Ok(with_status(
            format!("Zero or more users found {:?}", users),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        let user = users.get(0).unwrap();

        // check password
        type HmacSha256 = Hmac<Sha256>;
        let salt = user.salt.to_string();
        // "pSMrnnFNdJtanr5m+D2ZNHOpszE0sYFTAMWXkLfvR7F0euELeyEu1Q1AqwS7o3RTrHyo0UdYtwexDWe7N3gEyA==";
        let decoded_salt = general_purpose::STANDARD
            .decode(salt)
            .expect("Could note decode salt");
        let hashpassword = user.hashpassword.to_string();
        //"0mhcQnHQjB02bWs9J1u5WFD7e9qZnq32GWfsZjO/XlA=";
        let password = item.password;

        let mut mac = HmacSha256::new_from_slice(decoded_salt.as_slice())
            .expect("HMAC can take key of any size");
        mac.update(password.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let encoded: String = general_purpose::STANDARD.encode(code_bytes);

        if encoded != hashpassword {
            println!("Incorrect password for user {}", item.username);
            Ok(with_status(
                format!("Incorrect password {:?}", user),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        } else {
            println!("Logon success for user {}", item.username);
            let id_claims = IdClaims {
                username: user.username.to_string(),
                email: user.emailaddress.to_string(),
            };
            let claims =
                Claims::with_custom_claims(id_claims, coarsetime::Duration::from_secs(60 * 60 * 2))
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
}
