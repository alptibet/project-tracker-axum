use crate::appstate::AppState;
use crate::errors::AppError;
use crate::models::auth::Claims;
use crate::models::users::{User, UserDocument, UserRole, ValidUser};
use crate::utils::parse_oid;
use axum::Extension;
use axum::{
    extract::{State, TypedHeader},
    headers,
    http::Request,
    middleware::Next,
    response::Response,
};
use bcrypt::{verify, BcryptError};
use chrono::Utc;
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
    let exp = OffsetDateTime::now_utc() + Duration::hours(1);
    Cookie::build("token", sign_token(_id))
        .path("/")
        .secure(true)
        .same_site(tower_cookies::cookie::SameSite::None)
        .http_only(true)
        .expires(exp)
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

pub async fn match_auth(db: &Database, username: &str) -> mongodb::error::Result<Option<User>> {
    let collection = db.collection::<UserDocument>("users");
    let user_doc = collection
        .find_one(doc! {"username":username}, None)
        .await?;
    if user_doc.is_none() {
        return Ok(None);
    }
    let unwrapped_doc = user_doc.unwrap();
    let match_auth = User {
        _id: unwrapped_doc._id.to_string(),
        name: unwrapped_doc.name,
        surname: unwrapped_doc.surname,
        username: unwrapped_doc.username,
        email: unwrapped_doc.email,
        active: unwrapped_doc.active,
        password: unwrapped_doc.password,
        passwordChangeAt: unwrapped_doc.passwordChangeAt.to_string(),
        role: match unwrapped_doc.role {
            UserRole::Admin => "Admin".to_string(),
            UserRole::User => "User".to_string(),
            UserRole::Superuser => "Superuser".to_string(),
        },
    };

    Ok(Some(match_auth))
}

pub async fn authenticate_user<B>(
    State(state): State<AppState>,
    cookies: Option<TypedHeader<headers::Cookie>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let token: Option<String>;
    let auth_bearer = req.headers().get("authorization");
    if auth_bearer.is_none() && cookies.is_none() {
        return Err(AppError::NoAuth);
    }

    if let Some(_auth_bearer) = auth_bearer {
        let bearer: Vec<&str> = auth_bearer.unwrap().to_str().unwrap().split(' ').collect();
        token = Some(bearer[1].to_string());
    } else {
        token = Some(cookies.unwrap().get("token").unwrap().to_string());
    } //Shall we do error handling here?
    if let Some(user) = is_valid_user(&state.db, token).await {
        if !user.active {
            return Err(AppError::UserNotActive);
        }
        req.extensions_mut().insert(user);
        let response = next.run(req).await;
        Ok(response)
    } else {
        Err(AppError::NoAuth)
    }
}

async fn is_valid_user(db: &Database, token: Option<String>) -> Option<ValidUser> {
    let secret_key = env::var("JWT_SECRET").expect("No JWT KEY found in environment.");
    let payload = decode::<Claims>(
        token.unwrap().as_str(),
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    if let Ok(_payload) = payload {
        let oid = parse_oid(_payload.claims.sub).unwrap();
        match match_user(db, oid).await {
            Ok(_valid_user) => _valid_user,
            Err(_err) => None,
        }
    } else {
        None
    }
}

async fn match_user(db: &Database, oid: ObjectId) -> mongodb::error::Result<Option<ValidUser>> {
    let collection = db.collection::<UserDocument>("users");

    let user_doc = collection.find_one(doc! {"_id":oid}, None).await?;
    if user_doc.is_none() {
        return Ok(None);
    }

    let unwrapped_doc = user_doc.unwrap();
    let user_json = ValidUser {
        _id: unwrapped_doc._id.to_string(),
        username: unwrapped_doc.username,
        name: unwrapped_doc.name,
        surname: unwrapped_doc.surname,
        email: unwrapped_doc.email,
        active: unwrapped_doc.active,
        role: match unwrapped_doc.role {
            UserRole::Admin => "Admin".to_string(),
            UserRole::User => "User".to_string(),
            UserRole::Superuser => "Superuser".to_string(),
        },
    };

    Ok(Some(user_json))
}

pub async fn authorize_admin<B>(
    Extension(user): Extension<ValidUser>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    if user.role == "Admin" {
        let response = next.run(req).await;
        Ok(response)
    } else {
        Err(AppError::NotAuthorized)
    }
}
