use crate::appstate::AppState;
use crate::controllers::auth;
use crate::controllers::users;
use crate::errors::AppError;
use crate::models::auth::{NewUser, UserLogin};
use crate::models::response::DocResponse;
use crate::models::response::MessageResponse;

use crate::models::users::ValidUser;
use axum::extract::{Json, State};
use tower_cookies::Cookies;

pub async fn signup(
    State(state): State<AppState>,
    cookies: Cookies,
    input: NewUser,
) -> Result<Json<DocResponse<ValidUser>>, AppError> {
    match users::insert_one(&state.db, Json(input)).await {
        Ok(_user_doc) => {
            match auth::match_auth(&state.db, &_user_doc.username).await {
                Ok(auth_info) => {
                    if auth_info.is_none() {
                        return Err(AppError::BadRequest);
                    }
                    let token = auth::create_send_token(&auth_info.unwrap()._id);
                    cookies.add(token);
                }
                Err(_error) => (),
            }
            let user_json = ValidUser {
                _id: _user_doc._id,
                name: _user_doc.name,
                surname: _user_doc.surname,
                username: _user_doc.username,
                email: _user_doc.email,
                active: _user_doc.active,
                role: _user_doc.role,
            };
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: user_json,
            }))
        }
        Err(_error) => {
            let error = _error.kind.to_string();
            if error.contains("username_1") {
                return Err(AppError::UserAlreadyExists);
            }
            if error.contains("email_1") {
                return Err(AppError::EmailAlreadyExists);
            }
            Err(AppError::BadRequest)
        }
    }
}

pub async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(input): Json<UserLogin>,
) -> Result<Json<DocResponse<ValidUser>>, AppError> {
    let match_auth = match auth::match_auth(&state.db, &input.username).await {
        Ok(match_auth) => {
            if match_auth.is_none() {
                return Err(AppError::UserDoesNotExist);
            }
            Ok(match_auth.unwrap())
        }
        Err(_error) => Err(AppError::BadRequest),
    };
    let user = match_auth.unwrap();
    match auth::check_password(&input.password, &user.password) {
        Ok(passmatch) => {
            if passmatch {
                cookies.add(auth::create_send_token(&user._id));
                let user_json = ValidUser {
                    _id: user._id,
                    name: user.name,
                    surname: user.surname,
                    username: user.username,
                    email: user.email,
                    active: user.active,
                    role: user.role,
                };
                Ok(Json(DocResponse {
                    status: "success".to_string(),
                    data: user_json,
                }))
            } else {
                Err(AppError::WrongCredentials)
            }
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}

pub async fn logout(cookies: Cookies) {
    let token = auth::disable_token();
    cookies.add(token);
}
