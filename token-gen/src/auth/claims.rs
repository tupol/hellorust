use derive_more::Display;
use serde_with::skip_serializing_none;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde_json::Value;

use crate::auth::user::UserInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdClaims {
    pub name: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub at_hash: Option<String>,
}
impl IdClaims {
    pub fn from_user_info(user_info: UserInfo) -> Self {
        IdClaims {
            name: user_info.name,
            email: user_info.email_address,
            first_name: user_info.first_name,
            last_name: user_info.last_name,
            at_hash: None,
        }
    }
    pub fn with_at_hash(self, at_hash: String) -> Self {
        IdClaims{
            name: self.name,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            at_hash: Some(at_hash)
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaim {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
}
impl JwtClaim {
    pub fn empty() -> JwtClaim {
        JwtClaim {
            iss: None,
            sub: None,
            aud: None,
            exp: None,
            nbf: None,
            iat: None,
            jti: None,
        }
    }
    pub fn with_issuer(self, issuer: String) -> Self {
        JwtClaim{ iss: Some(issuer), sub: self.sub, aud: self.aud, exp: self.exp, nbf: self.nbf, iat: self.iat, jti: self.jti }
    }
    pub fn with_subject(self, subject: String) -> Self {
        JwtClaim{ iss: self.iss, sub: Some(subject), aud: self.aud, exp: self.exp, nbf: self.nbf, iat: self.iat, jti: self.jti }
    }
    pub fn with_audience(self, audience: String) -> Self {
        JwtClaim{ iss: self.iss, sub: self.sub, aud: Some(audience), exp: self.exp, nbf: self.nbf, iat: self.iat, jti: self.jti }
    }
    pub fn expires_at(self, seconds_since_epoch: u64) -> Self {
        JwtClaim{ iss: self.iss, sub: self.sub, aud: self.aud, exp: Some(seconds_since_epoch), nbf: self.nbf, iat: self.iat, jti: self.jti }
    }
    pub fn expires_in(self, seconds: u64) -> Self {
        JwtClaim{ iss: self.iss, sub: self.sub, aud: self.aud, exp: Some(duration_since_epoch().as_secs() + seconds), nbf: self.nbf, iat: self.iat, jti: self.jti }
    }
    pub fn starts_at(self, seconds_since_epoch: u64) -> Self {
        JwtClaim{ iss: self.iss, sub: self.sub, aud: self.aud, exp: self.exp, nbf: Some(seconds_since_epoch), iat: self.iat, jti: self.jti }
    }
    pub fn starts_now(self) -> Self {
        JwtClaim{ iss: self.iss, sub: self.sub, aud: self.aud, exp: self.exp, nbf: Some(duration_since_epoch().as_secs()), iat: self.iat, jti: self.jti }
    }
    pub fn issued_at(self, seconds_since_epoch: u64) -> Self {
        JwtClaim{ iss: self.iss, sub: self.sub, aud: self.aud, exp: self.exp, nbf: self.nbf, iat: Some(seconds_since_epoch), jti: self.jti }
    }
    pub fn issued_now(self) -> Self {
        JwtClaim{ iss: self.iss, sub: self.sub, aud: self.aud, exp: self.exp, nbf: self.nbf, iat: Some(duration_since_epoch().as_secs()), jti: self.jti }
    }
    pub fn with_id(self, id: String) -> Self {
        JwtClaim{ iss: self.iss, sub: self.sub, aud: self.aud, exp: self.exp, nbf: self.nbf, iat: self.iat, jti: Some(id) }
    }
    pub fn with_content<T: Serialize>(self, content: T) -> JwtClaimWithContent<T> {
        JwtClaimWithContent{
            content: content,
            claim: self
        }
    }
}

pub struct JwtClaimWithContent<T: Serialize> {
    content: T,
    claim: JwtClaim
}
impl<T: Serialize> JwtClaimWithContent<T> {
    pub fn as_json_value(&self) -> Result<Value, String> {
        self.as_json()
            .and_then(|x| serde_json::from_str::<Value>(&x).map_err(|er| er.to_string()))

    }
    pub fn as_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.claim)
            .map_err(|e| e.to_string())
            .and_then(|cl| serde_json::to_string(&self.content)
                .map_err(|e| e.to_string())
                .map(|co| merge_json(&cl, &co))
            )

    }
}


fn duration_since_epoch() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("This is the time before time.")
}

fn merge_json(first: &str, second: &str) -> String {
    let ft = first.trim();
    let st = second.trim();
    if(ft.is_empty()) {
        st.to_string()
    } else if (st.is_empty()) {
        ft.to_string()
    } else {
        let f = &ft[..ft.len()-1];
        let s = &st[1..];
        let mut res: String = f.to_string().to_owned();
        res.push_str(",");
        res.push_str(s);
        res
    }
}