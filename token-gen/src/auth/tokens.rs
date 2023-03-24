use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::auth::claims::{AccessClaims, IdClaims, JwtClaim};
use crate::auth::config::AuthConfig;
use crate::auth::user::UserInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdToken {
    pub header: Header,
    pub claims: JwtClaim,
    pub content: IdClaims,
}
impl IdToken {
    pub fn raw_token(&self, conf: &AuthConfig) -> Result<String, String> {
        create_id_token(conf, &self.header, &self.content)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub header: Header,
    pub claims: JwtClaim,
    pub content: AccessClaims
}
impl AccessToken {
    pub fn raw_token(&self, conf: &AuthConfig) -> Result<String, String> {
        create_access_token(conf, &self.header, &self.content)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub id_token: IdToken,
    pub access_token: AccessToken,
}
impl TokenPair{
    pub fn create(conf: &AuthConfig, header: &Header, user_info: UserInfo, session_id: String) -> Result<TokenPair, String> {
        fn claims(conf: &AuthConfig) -> JwtClaim {
            default_claims()
                .with_audience(conf.audience.clone())
                .with_issuer(conf.issuer.clone())
        }
        let access_claims = AccessClaims{session_id};
        let access_token = AccessToken{ header: header.clone(), claims: claims(conf), content: access_claims };
        let at_hash = access_token
            .raw_token(conf)
            .map_err(|er| er.to_string())
            .and_then(|at| hash_token(&at));
        let id_claims = at_hash
            .map(|at| IdClaims::from_user_info(user_info).with_at_hash(at));
        let id_token = id_claims.map(|c| IdToken{ header: header.clone(), claims: claims(conf), content: c });
        id_token.map(|idt| TokenPair{ id_token: idt, access_token: access_token})
    }
}

fn create_access_token(conf: &AuthConfig, header: &Header, content: &AccessClaims) -> Result<String, String> {
    create_token(conf, header, content).map_err(|er| er.to_string())
}

fn create_id_token(conf: &AuthConfig, header: &Header, content: &IdClaims) -> Result<String, String> {
    create_token(conf, header, content).map_err(|er| er.to_string())
}

fn default_claims() -> JwtClaim {
    JwtClaim::empty()
    .issued_now()
    .expires_in(10 * 60)
}

fn create_token<T: Serialize>(conf: &AuthConfig, header: &Header, content: T) -> Result<String, Error> {

    let claims = default_claims()
        .with_content(content)
        .as_json_value();
    let token = encode(
        &header,
        &claims.unwrap(),
        &conf.key,
    );
    return  token;
}

fn base64_encode(input: &str) -> Result<String, String> {
    base64_encode_u8(input.as_bytes())
        .and_then(|x| String::from_utf8(x)
            .map_err(|er| er.to_string()))
}

fn base64_encode_u8(input: &[u8]) -> Result<Vec<u8>, String> {
    use base64::{Engine as _, engine::general_purpose};
    let mut buf = Vec::new();
    buf.resize(input.len() * 4 / 3 + 4, 0);

    general_purpose::STANDARD
        .encode_slice(input, &mut buf)
        .map_err(|er| er.to_string())
        .map(|bytes_written| buf.truncate(bytes_written))
        .map(|_| buf)
}

fn sha256(input: &[u8]) -> Vec<u8> {
    use sha2::{Sha256, Sha512, Digest};

    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    result[..].to_vec()
}

fn hash_token(input: &str) -> Result<String, String> {
    base64_encode(input)
        .and_then(|x| base64_encode_u8(&x.as_bytes()))
        .and_then(|x| {
            let s = sha256(&x);
            let l = s.len()/2;
            let z = s.split_at(l).0;
            base64_encode_u8(&z)
        })
        .and_then(|x| String::from_utf8(x).map_err(|e| e.to_string()))
}

