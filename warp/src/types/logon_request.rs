use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogonRequest {
    pub username: String,
    pub password: String,
}
