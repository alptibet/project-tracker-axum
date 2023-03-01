use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Json},
    http::Request,
    http::StatusCode,
    BoxError, RequestExt,
};
use mongodb::bson::datetime::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use validator::Validate;

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
    pub active: bool,
    pub password: String,
    pub passwordChangeAt: String,
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
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ValidUser {
    pub _id: String,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub active: bool,
    pub role: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct Me {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    pub name: String,
    #[validate(length(min = 2, message = "Surname must be at least 2 characters long"))]
    pub surname: String,
    #[validate(length(min = 4, message = "Username must be at least 4 characters long"))]
    pub username: String,
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    pub active: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UserId {
    pub _id: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserUpdate {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    pub name: String,
    #[validate(length(min = 2, message = "Surname must be at least 2 characters long"))]
    pub surname: String,
    #[validate(length(min = 4, message = "Username must be at least 4 characters long"))]
    pub username: String,
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    pub active: bool,
    pub role: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for UserUpdate
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req.extract::<Json<UserUpdate>, _>().await.unwrap();
        if let Err(errors) = user.validate() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status":"validation error",
                    "errors": errors
                })),
            ));
        }
        Ok(user)
    }
}

// #[derive(Deserialize, Serialize, Validate)]
// pub struct MeUpdate {
//     #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
//     pub name: String,
//     #[validate(length(min = 2, message = "Surname must be at least 2 characters long"))]
//     pub surname: String,
//     #[validate(length(min = 4, message = "Username must be at least 4 characters long"))]
//     pub username: String,
//     #[validate(email(message = "Enter a valid email address"))]
//     pub email: String,
//     pub active: bool,
// }

#[async_trait]
impl<S, B> FromRequest<S, B> for Me
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req.extract::<Json<Me>, _>().await.unwrap();
        if let Err(errors) = user.validate() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status":"validation error",
                    "errors": errors
                })),
            ));
        }
        Ok(user)
    }
}
