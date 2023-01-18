use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub scope: String,
    pub expires_in: i32,
    pub token_type: String,
}
