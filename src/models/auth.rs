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

#[derive(Deserialize, Serialize)]
pub struct AuthInfo {
    pub _id: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserId {
    pub _id: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct UserInput {
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub passwordConfirm: String,
}
