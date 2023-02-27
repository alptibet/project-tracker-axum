use crate::appstate::AppState;
use crate::controllers::auth;
use crate::controllers::users;
use crate::errors::AppError;
use crate::models::auth::{UserInput, UserLogin};
use crate::models::response::MessageResponse;
use axum::extract::{Json, State};
use tower_cookies::Cookies;

pub async fn signup(
    State(state): State<AppState>,
    cookies: Cookies,
    input: UserInput,
) -> Result<Json<MessageResponse>, AppError> {
    match users::insert_one(&state.db, Json(input)).await {
        Ok(_user_doc) => {
            match auth::match_auth(&state.db, &_user_doc.username).await {
                Ok(_auth_info) => {
                    if _auth_info.is_none() {
                        return Err(AppError::BadRequest);
                    }
                    let token = auth::create_send_token(&_auth_info.unwrap()._id);
                    cookies.add(token);
                }
                Err(_error) => (),
            }
            Ok(Json(MessageResponse {
                message: "success".to_string(),
            }))
        }
        Err(_error) => {
            let res = _error.to_string();
            if res.contains("code: 11000") {
                return Err(AppError::UserAlreadyExists);
            }
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(input): Json<UserLogin>,
) -> Result<Json<MessageResponse>, AppError> {
    let match_auth = match auth::match_auth(&state.db, &input.username).await {
        Ok(_match_auth) => {
            if _match_auth.is_none() {
                return Err(AppError::UserDoesNotExist);
            }
            Ok(_match_auth.unwrap())
        }
        Err(_error) => Err(AppError::BadRequest),
    };
    let auth_unwrapped = match_auth.unwrap();
    match auth::check_password(&input.password, &auth_unwrapped.password) {
        Ok(_match) => {
            if _match {
                cookies.add(auth::create_send_token(&auth_unwrapped._id));
                Ok(Json(MessageResponse {
                    message: "Logged in successfully".to_string(),
                }))
            } else {
                Err(AppError::WrongCredentials)
            }
        }
        Err(_error) => Err(AppError::InternalServerError),
    }
}

pub async fn logout(cookies: Cookies) {
    let token = auth::disable_token();
    cookies.add(token);
}
