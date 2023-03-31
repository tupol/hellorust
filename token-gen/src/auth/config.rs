use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;

pub struct AuthConfig {
    pub encoding_key: EncodingKey,
    pub decoding_key: Option<DecodingKey>,
    pub issuer: String,
    pub audience: String,
}
