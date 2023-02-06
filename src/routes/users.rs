use crate::appstate::AppState;
use crate::controllers::users;
use crate::errors::AppError;
use crate::models::response::{DocResponse, MessageResponse, VecResponse};
use crate::models::users::{User, UserInput, UserId};
use axum::extract::{Json, Path, State};
use mongodb::bson::oid::ObjectId;
use cookie::{Cookie, CookieJar};

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<User>>, AppError> {
    match users::get_all(&state.db).await {
        Ok(_users_doc) => Ok(Json(VecResponse {
            message: "Success".to_string(),
            data: _users_doc,
        })),
        Err(_error) => Err(AppError::NotFound),
    }
}

pub async fn get_one_user(
    Path(_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<DocResponse<User>>, AppError> {
    let oid = ObjectId::parse_str(_id);
    if oid.is_err() {
        return Err(AppError::OidParseError);
    }
    match users::find_one(&state.db, oid.unwrap()).await {
        Ok(_user_doc) => {
            if _user_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                message: "success".to_string(),
                data: _user_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    input: Json<UserId>
) -> Result<Json<MessageResponse>, AppError> {
    let oid = ObjectId::parse_str(input._id.clone());
    if oid.is_err() {
        return Err(AppError::OidParseError);
    }
    match users::deactivate_user(&state.db, oid.unwrap()).await {
        Ok(_user_doc) => {
            if _user_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(MessageResponse {
                message: "Success".to_string(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}

pub async fn signup(
    State(state): State<AppState>,
    input: Json<UserInput>,
) -> Result<Json<DocResponse<User>>, AppError> {
    match users::insert_one(&state.db, input).await {
        Ok(_user_doc) => Ok(Json(DocResponse {
            message: "Success".to_string(),
            data: _user_doc,
        })),
        Err(_error) => {
            let res = _error.to_string();
            if res.contains("code: 11000") {
                return Err(AppError::DuplicateRecord);
            }
            Err(AppError::BadRequest)
        }
    }
}
