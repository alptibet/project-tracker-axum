use crate::appstate::AppState;
use crate::controllers::materials;
use crate::errors::AppError;
use crate::models::response::{DocResponse, MessageResponse, VecResponse};
use crate::{
    models::materials::{Material, MaterialInput},
    utils::parse_oid,
};
use axum::extract::{Json, Path, State};
use axum::routing::{delete, get, patch, post};
use axum::Router;

pub fn create_materials_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_materials))
        .route("/:id", get(get_one_material))
        .route("/", post(insert_material))
        .route("/:id", delete(delete_material))
        .route("/:id", patch(update_material))
}

pub async fn get_all_materials(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<Material>>, AppError> {
    match materials::get_all(&state.db).await {
        Ok(material_doc) => Ok(Json(VecResponse {
            status: "success".to_string(),
            data: material_doc,
        })),
        Err(_error) => Err(AppError::NotFound),
    }
}

pub async fn get_one_material(
    Path(_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<DocResponse<Material>>, AppError> {
    let oid = parse_oid(_id)?;
    match materials::get_one(&state.db, oid).await {
        Ok(material_doc) => {
            if material_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: material_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::NotFound),
    }
}

pub async fn insert_material(
    State(state): State<AppState>,
    input: MaterialInput,
) -> Result<Json<DocResponse<Material>>, AppError> {
    match materials::insert_one(&state.db, Json(input)).await {
        Ok(material_doc) => Ok(Json(DocResponse {
            status: "success".to_string(),
            data: material_doc,
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

pub async fn delete_material(
    Path(_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<MessageResponse>, AppError> {
    let oid = parse_oid(_id)?;
    match materials::delete_one(&state.db, oid).await {
        Ok(material_doc) => {
            if material_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(MessageResponse {
                status: "success".to_string(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}

pub async fn update_material(
    Path(_id): Path<String>,
    State(state): State<AppState>,
    input: MaterialInput,
) -> Result<Json<DocResponse<Material>>, AppError> {
    let oid = parse_oid(_id)?;
    match materials::update_one(&state.db, oid, Json(input)).await {
        Ok(material_doc) => {
            if material_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: material_doc.unwrap(),
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
