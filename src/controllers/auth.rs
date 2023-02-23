use crate::appstate::AppState;
use crate::errors::AppError;
use crate::models::auth::{AuthInfo, Claims, UserId};
use crate::models::users::UserDocument;
use axum::extract::State;
use axum::{extract::TypedHeader, http::Request, middleware::Next, response::Response};
use bcrypt::{verify, BcryptError};
use chrono::Utc;
use headers;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
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
    let exp = OffsetDateTime::now_utc() + Duration::seconds(100);
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
    State(state): State<AppState>,
    TypedHeader(cookie): TypedHeader<headers::Cookie>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let token: Option<String>;
    let auth_bearer = req.headers().get("authorization");
    let cookie_val = cookie.get("token");
    if auth_bearer.is_none() && cookie_val.is_none() {
        return Err(AppError::NotAuthorized);
    }

    if let Some(_auth_bearer) = auth_bearer {
        let bearer: Vec<&str> = auth_bearer.unwrap().to_str().unwrap().split(' ').collect();
        token = Some(bearer[1].to_string());
    } else {
        token = Some(cookie_val.unwrap().to_string());
    } //Shall we do error handling here?
    if is_valid_token(&state.db, token).await {
        let response = next.run(req).await;
        Ok(response)
    } else {
        Err(AppError::NotAuthorized)
    }
}

async fn is_valid_token(db: &Database, token: Option<String>) -> bool {
    let secret_key = env::var("JWT_SECRET").expect("No JWT KEY found in environment.");
    let payload = decode::<Claims>(
        token.unwrap().as_str(),
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    if let Ok(_payload) = payload {
        let oid = ObjectId::parse_str(_payload.claims.sub).unwrap();
        match match_user_id(db, oid).await {
            Ok(_valid_user) => {
                if _valid_user.is_none() {
                    return false;
                }
                return true;
            }
            Err(_err) => false,
        };
        true
    } else {
        false
    }
}

async fn match_user_id(db: &Database, oid: ObjectId) -> mongodb::error::Result<Option<UserId>> {
    let collection = db.collection::<UserId>("users");
    let user_doc = collection.find_one(doc! {"_id":oid}, None).await?;
    if user_doc.is_none() {
        return Ok(None);
    }
    let unwrapped_doc = user_doc.unwrap();
    let user_json = UserId {
        _id: unwrapped_doc._id,
    };
    Ok(Some(user_json))
}
