use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogonRequest {
    username: String,
    password: String,
}
