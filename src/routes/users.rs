use crate::appstate::AppState;
use crate::controllers::users;
use crate::errors::AppError;
use crate::models::response::{DocResponse, VecResponse};
use crate::models::users::{User, UserUpdate};
use axum::extract::{Json, Path, State};
use mongodb::bson::oid::ObjectId;

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

pub async fn update_user(
    Path(_id): Path<String>,
    State(state): State<AppState>,
    input: Json<UserUpdate>,
) -> Result<Json<DocResponse<User>>, AppError> {
    let oid = ObjectId::parse_str(_id);
    if oid.is_err() {
        return Err(AppError::OidParseError);
    }
    match users::update_one(&state.db, oid.unwrap(), input).await {
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
