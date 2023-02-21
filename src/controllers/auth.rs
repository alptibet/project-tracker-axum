use crate::models::auth::{AuthInfo, Claims};
use crate::models::users::UserDocument;
use axum::{
    extract::TypedHeader,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use bcrypt::{verify, BcryptError};
use chrono::Utc;
use headers;
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::doc;
use mongodb::Database;
use std::env;
use tower_cookies::{
    cookie::time::{Duration, OffsetDateTime},
    Cookie,
};

pub fn create_send_token<'a>(_id: &str) -> Cookie<'a> {
    Cookie::build("token", sign_token(_id))
        .path("/")
        .secure(false)
        .http_only(true)
        .finish()
}

pub fn sign_token(_id: &str) -> String {
    let secret_key = env::var("JWT_SECRET").expect("No JWT KEY found in environment.");
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid time stamp")
        .timestamp();

    let my_claim = Claims {
        sub: _id.to_string(),
        exp: expiration as usize,
    };

    let result = encode::<Claims>(
        &Header::default(),
        &my_claim,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .unwrap();
    result
}

pub fn check_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    verify(password, hashed_password)
}

pub fn disable_token<'a>() -> Cookie<'a> {
    let exp = OffsetDateTime::now_utc() + Duration::seconds(10);
    Cookie::build("token", "byebye")
        .path("/")
        .secure(false)
        .http_only(true)
        .expires(exp)
        .finish()
}

pub async fn match_auth(db: &Database, username: &str) -> mongodb::error::Result<Option<AuthInfo>> {
    let collection = db.collection::<UserDocument>("users");
    let user_doc = collection
        .find_one(doc! {"username":username}, None)
        .await?;
    if user_doc.is_none() {
        return Ok(None);
    }
    let unwrapped_doc = user_doc.unwrap();
    let match_auth = AuthInfo {
        _id: unwrapped_doc._id.to_string(),
        password: unwrapped_doc.password,
    };

    Ok(Some(match_auth))
}

pub async fn authenticate_user<B>(
    TypedHeader(cookie): TypedHeader<headers::Cookie>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let value = cookie.get("token");
    println!("{value:?}");
    let bearer = request.headers().get("authorization");
    println!("{bearer:?}");
    let response = next.run(request).await;
    Ok(response)
}
