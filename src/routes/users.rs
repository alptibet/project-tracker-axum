use crate::appstate::AppState;
use crate::controllers::users;
use crate::errors::AppError;
use crate::models::response::{DocResponse, VecResponse};
use crate::models::users::{Me, User, UserUpdate, ValidUser};
use crate::utils::parse_oid;
use axum::extract::{Json, Path, State};
use axum::Extension;

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
    let oid = parse_oid(_id)?;
    match users::find_one(&state.db, oid).await {
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
    input: UserUpdate,
) -> Result<Json<DocResponse<User>>, AppError> {
    let oid = parse_oid(_id)?;
    match users::update_one(&state.db, oid, Json(input)).await {
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

pub async fn get_me(
    State(state): State<AppState>,
    Extension(user): Extension<ValidUser>,
) -> Result<Json<DocResponse<Me>>, AppError> {
    let oid = parse_oid(user._id)?;
    match users::get_me(&state.db, oid).await {
        Ok(_user_doc) => {
            if _user_doc.is_none() {
                return Err(AppError::BadRequest); //Change this with something like it is not you
            }
            Ok(Json(DocResponse {
                message: "success".to_string(),
                data: _user_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}
