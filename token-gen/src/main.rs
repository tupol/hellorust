extern crate core;

mod auth;

use jsonwebtoken::{Algorithm, EncodingKey, Header, DecodingKey};

use crate::auth::config::AuthConfig;
use crate::auth::tokens::{AccessToken, IdToken, TokenPair};
use crate::auth::user::UserInfo;
use crate::auth::claims::{AccessClaims, JwtClaim};

fn main() {

    let encoding_key = EncodingKey::from_rsa_pem(include_bytes!("../privatekey.pkcs8")).unwrap();
    let decoding_key = DecodingKey::from_rsa_pem(include_bytes!("../publickey.pem")).unwrap();

    let auth_conf = AuthConfig {
        encoding_key,
        decoding_key: Some(decoding_key),
        issuer: "my_issuer".to_string(),
        audience: "my_audience".to_string(),
    };
    let header = Header::new(Algorithm::RS256);

    let user_info = UserInfo {
        name: "test_user".to_string(),
        email_address: "test_email_address".to_string(),
        first_name: "test_first_name".to_string(),
        last_name: "test_last_name".to_string(),
    };

    let session_id = "some_id".to_string();

    let tp = TokenPair::create(&auth_conf, &header, user_info, session_id).unwrap();

    println!("{:?}", tp.id_token);
    println!("{:?}", tp.access_token);

    println!(
        "ID Token     {}",
        tp.id_token.raw_token(&auth_conf).unwrap()
    );
    println!(
        "Access Token {}",
        tp.access_token.raw_token(&auth_conf).unwrap()
    );

    let raw_id_token = tp.id_token.raw_token(&auth_conf).unwrap();
    let raw_access_token = tp.access_token.raw_token(&auth_conf).unwrap();

    println!("{:?}", IdToken::from_raw_token(&auth_conf, &raw_id_token).unwrap());
    println!("{:?}", AccessToken::from_raw_token(&auth_conf, &raw_access_token).unwrap());

    let tp = TokenPair::from_raw_tokens(&auth_conf, &raw_id_token, &raw_access_token).unwrap();
    println!("{:?}", tp.id_token);
    println!("{:?}", tp.access_token);

    let unknown_access_token = AccessToken{
        header,
        claims: JwtClaim::empty(),
        content: AccessClaims{ session_id: "".to_string() }
    };

    let failed_extraction_result = TokenPair::from_raw_tokens(&auth_conf, &raw_id_token, &unknown_access_token.raw_token(&auth_conf).unwrap());
    assert!(failed_extraction_result.is_err())

}

