use crate::controllers::systems;
use crate::errors::AppError;
use crate::models::response::{DocResponse, MessageResponse, VecResponse};
use crate::models::systems::SystemInput;
use crate::utils::parse_oid;
use crate::{appstate::AppState, models::systems::System};
use axum::extract::{Json, Path, State};
use axum::routing::{delete, get, patch, post};
use axum::Router;

pub fn create_systems_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_systems))
        .route("/:id", get(get_one_system))
        .route("/", post(insert_system))
        .route("/:id", delete(delete_system))
        .route("/:id", patch(update_system))
}

pub async fn get_all_systems(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<System>>, AppError> {
    match systems::get_all(&state.db).await {
        Ok(systems_doc) => Ok(Json(VecResponse {
            status: "success".to_string(),
            data: systems_doc,
        })),
        Err(_error) => Err(AppError::NotFound),
    }
}

pub async fn get_one_system(
    Path(_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<DocResponse<System>>, AppError> {
    let oid = parse_oid(_id)?;
    match systems::get_one(&state.db, oid).await {
        Ok(system_doc) => {
            if system_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: system_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::NotFound),
    }
}

pub async fn insert_system(
    State(state): State<AppState>,
    input: SystemInput,
) -> Result<Json<DocResponse<System>>, AppError> {
    match systems::insert_one(&state.db, Json(input)).await {
        Ok(_system_doc) => Ok(Json(DocResponse {
            status: "success".to_string(),
            data: _system_doc,
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

pub async fn delete_system(
    Path(_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<MessageResponse>, AppError> {
    let oid = parse_oid(_id)?;
    match systems::delete_one(&state.db, oid).await {
        Ok(system_doc) => {
            if system_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(MessageResponse {
                status: "success".to_string(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}

pub async fn update_system(
    Path(_id): Path<String>,
    State(state): State<AppState>,
    input: SystemInput,
) -> Result<Json<DocResponse<System>>, AppError> {
    let oid = parse_oid(_id)?;
    match systems::update_one(&state.db, oid, Json(input)).await {
        Ok(system_doc) => {
            if system_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: system_doc.unwrap(),
            }))
        }
        Err(error) => {
            let res = error.to_string();
            if res.contains("E11000") {
                return Err(AppError::DuplicateRecord);
            }
            Err(AppError::BadRequest)
        }
    }
}
