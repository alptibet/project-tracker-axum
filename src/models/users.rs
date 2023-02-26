use mongodb::bson::datetime::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum UserRole {
    User,
    Admin,
    Superuser,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct User {
    pub _id: String,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub active: String,
    pub password: String,
    pub passwordChangeAt: String,
    pub role: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ValidUser {
    pub _id: String,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub active: String,
    pub role: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserUpdate {
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub active: String,
    pub role: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct UserDocument {
    pub _id: ObjectId,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub active: bool,
    pub password: String,
    pub passwordChangeAt: DateTime,
    pub role: UserRole,
}

#[derive(Deserialize, Serialize)]
pub struct UserId {
    pub _id: String,
}
