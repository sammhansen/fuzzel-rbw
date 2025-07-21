use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub username: String,
    pub password: String,
    pub totp: Option<String>,
}
