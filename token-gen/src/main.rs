extern crate core;

mod auth;

use std::ops::Deref;
use jsonwebtoken::{Algorithm, EncodingKey, Header};

use crate::auth::config::AuthConfig;
use crate::auth::tokens::TokenPair;
use crate::auth::user::UserInfo;

fn main() {

    let key = EncodingKey::from_rsa_pem(include_bytes!("../privatekey-authx.pkcs8")).unwrap();
    let auth_conf = AuthConfig{ key: key, issuer: "my_issuer".to_string(), audience: "my_audience".to_string()};
    let header = Header::new(Algorithm::RS256);

    let user_info = UserInfo{
        name: "test_user".to_string(),
        email_address: "test_email_address".to_string(),
        first_name: "test_first_name".to_string(),
        last_name: "test_last_name".to_string(),
    };

    let session_id = "some_id".to_string();

    let tp = TokenPair::create(&auth_conf, &header, user_info, session_id).unwrap();
    println!("ID Token     {}", tp.id_token.raw_token(&auth_conf).unwrap());
    println!("Access Token {}", tp.access_token.raw_token(&auth_conf).unwrap());

}
