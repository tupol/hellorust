use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub hashpassword: String,
    pub salt: String,
    pub loa1level: i32,
    pub loa2level: i32,
    pub amid: String,
    pub amstate: String,
    pub amlocktime: i32,
    pub name: String,
    pub emailaddress: String,
    pub typeuser: String,
    pub firstname: String,
    pub lastname: String,
    pub usertechnicalid: String,
    pub pwcreated: chrono::NaiveDate,
}
