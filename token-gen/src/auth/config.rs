use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;

pub struct AuthConfig {
    pub key: EncodingKey,
    pub issuer: String,
    pub audience: String,
}
