use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Json},
    http::Request,
    http::StatusCode,
    BoxError, RequestExt,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use typeshare::typeshare;
use validator::Validate;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[typeshare]
#[derive(Deserialize, Serialize, Validate)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthInfo {
    pub _id: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserId {
    pub _id: String,
}

#[typeshare]
#[allow(non_snake_case)]
#[derive(Deserialize, Validate)]
pub struct NewUser {
    pub name: String,
    pub surname: String,
    #[validate(length(min = 4, message = "Username must be at least 4 characters long"))]
    pub username: String,
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords must match"))]
    pub passwordConfirm: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for NewUser
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req.extract::<Json<NewUser>, _>().await.unwrap();
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
