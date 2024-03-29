use crate::appstate::AppState;
use crate::controllers::users;
use crate::errors::AppError;
use crate::models::response::{DocResponse, VecResponse};
use crate::models::users::{Me, User, UserUpdate, ValidUser};
use crate::utils::parse_oid;
use axum::extract::{Json, Path, State};
use axum::routing::{get, patch};
use axum::Extension;
use axum::Router;

pub fn create_users_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_users))
        .route("/:id", get(get_one_user))
        .route("/:id", patch(update_user))
}

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<User>>, AppError> {
    match users::get_all(&state.db).await {
        Ok(users_doc) => Ok(Json(VecResponse {
            status: "success".to_string(),
            data: users_doc,
        })),
        Err(_error) => Err(AppError::BadRequest),
    }
}

pub async fn get_one_user(
    Path(_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<DocResponse<User>>, AppError> {
    let oid = parse_oid(_id)?;
    match users::find_one(&state.db, oid).await {
        Ok(user_doc) => {
            if user_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: user_doc.unwrap(),
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
        Ok(user_doc) => {
            if user_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: user_doc.unwrap(),
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

pub async fn get_me(
    State(state): State<AppState>,
    Extension(user): Extension<ValidUser>,
) -> Result<Json<DocResponse<Me>>, AppError> {
    let oid = parse_oid(user._id)?;
    match users::get_me(&state.db, oid).await {
        Ok(user_doc) => {
            if user_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: user_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}

pub async fn update_me(
    State(state): State<AppState>,
    Extension(user): Extension<ValidUser>,
    input: Me,
) -> Result<Json<DocResponse<Me>>, AppError> {
    let oid = parse_oid(user._id.clone())?;
    match users::update_me(&state.db, Json(input), oid).await {
        Ok(user_doc) => {
            if user_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: user_doc.unwrap(),
            }))
        }
        Err(error) => {
            let error = error.kind.to_string();
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
