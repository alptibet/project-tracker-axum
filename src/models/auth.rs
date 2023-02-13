use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}
