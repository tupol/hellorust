use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    // pub username: String
    // pub hashpassword: String,
    // pub salt: String,
    // pub loa1level: i32,
    // pub loa2level: i32,
    // pub amid: String,
    // pub amstate: String,
    // pub amlocktime: i32,
    pub name: String,
    pub email_address: String,
    // pub type_user: String,
    pub first_name: String,
    pub last_name: String,
    // pub user_technical_id: String,
    // pub pwcreated: u64,
}
