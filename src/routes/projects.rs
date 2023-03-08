use crate::controllers::projects;
use crate::errors::AppError;
use crate::models::projects::{ProjectInput, ProjectUpdate};
use crate::models::response::{DocResponse, VecResponse};
use crate::utils::parse_oid;
use crate::{appstate::AppState, models::projects::Project};
use axum::extract::{Json, Path, State};

pub async fn get_all_projects(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<Project>>, AppError> {
    match projects::get_all(&state.db).await {
        Ok(_projects_doc) => Ok(Json(VecResponse {
            message: "Success".to_string(),
            data: _projects_doc,
        })),
        Err(_error) => {
            dbg!(_error);
            Err(AppError::BadRequest)
        }
    }
}

pub async fn get_one_project(
    State(state): State<AppState>,
    Path(_id): Path<String>,
) -> Result<Json<DocResponse<Project>>, AppError> {
    let oid = parse_oid(_id)?;
    match projects::get_one(&state.db, oid).await {
        Ok(_project_doc) => {
            if _project_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                message: "success".to_string(),
                data: _project_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}

pub async fn insert_project(
    State(state): State<AppState>,
    input: ProjectInput,
) -> Result<Json<DocResponse<Project>>, AppError> {
    match projects::insert_one(&state.db, Json(input)).await {
        Ok(_project_doc) => Ok(Json(DocResponse {
            message: "Success".to_string(),
            data: _project_doc,
        })),
        Err(_error) => {
            let res = _error.to_string();
            println!("{res:?}");
            if res.contains("code: 11000") {
                return Err(AppError::DuplicateRecord);
            }
            Err(AppError::BadRequest)
        }
    }
}

pub async fn update_project(
    Path(_id): Path<String>,
    State(state): State<AppState>,
    input: ProjectUpdate,
) -> Result<Json<DocResponse<Project>>, AppError> {
    let oid = parse_oid(_id)?;
    match projects::update_one(&state.db, oid, Json(input)).await {
        Ok(_project_doc) => {
            if _project_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                message: "success".to_string(),
                data: _project_doc.unwrap(),
            }))
        }
        Err(_error) => {
            dbg!(&_error);
            let error = _error.kind.to_string();
            if error.contains("name_1") {
                return Err(AppError::DuplicateRecord);
            }
            Err(AppError::BadRequest)
        }
    }
}
