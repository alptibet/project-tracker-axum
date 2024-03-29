use crate::appstate::AppState;
use crate::controllers::projects;
use crate::errors::AppError;
use crate::models::projects::{
    DeletedProject, MaterialToDelete, MaterialWithSysIndicator, ProjectInput, ProjectUpdate,
    ProjectWithMaterials, ProjectWithoutMaterials, UpdatedMaterials,
};
use crate::models::response::{DocResponse, VecResponse};
use crate::utils::parse_oid;
use axum::extract::{Json, Path, State};
use axum::routing::{delete, get, patch, post};
use axum::Router;

pub fn create_projects_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_projects))
        .route("/:id", get(get_one_project))
        .route("/", post(insert_project))
        .route("/:id", delete(delete_project))
        .route("/:id", patch(update_project))
        .route("/removematerial/:id", patch(remove_project_material))
        .route("/editmaterial/:id", patch(update_project_material))
        .route("/addmaterial/:id", patch(insert_project_material))
        .route("/detailed/:id", get(get_one_project_with_materials))
}
//Gets all projects without materials data
pub async fn get_all_projects(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<ProjectWithoutMaterials>>, AppError> {
    match projects::get_all(&state.db).await {
        Ok(projects_doc) => Ok(Json(VecResponse {
            status: "success".to_string(),
            data: projects_doc,
        })),
        Err(_error) => Err(AppError::BadRequest),
    }
}

//Gets one project without materials data
pub async fn get_one_project(
    State(state): State<AppState>,
    Path(_id): Path<String>,
) -> Result<Json<DocResponse<ProjectWithoutMaterials>>, AppError> {
    let oid = parse_oid(_id)?;
    match projects::get_one(&state.db, oid).await {
        Ok(project_doc) => {
            if project_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: project_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}

//Gets one project with materials data
pub async fn get_one_project_with_materials(
    State(state): State<AppState>,
    Path(_id): Path<String>,
) -> Result<Json<DocResponse<ProjectWithMaterials>>, AppError> {
    let oid = parse_oid(_id)?;
    match projects::get_one_with_materials(&state.db, oid).await {
        Ok(project_doc) => {
            if project_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: project_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}

//Inserts a project with empty materials array
pub async fn insert_project(
    State(state): State<AppState>,
    input: ProjectInput,
) -> Result<Json<DocResponse<ProjectWithMaterials>>, AppError> {
    match projects::insert_one(&state.db, Json(input)).await {
        Ok(project_doc) => Ok(Json(DocResponse {
            status: "success".to_string(),
            data: project_doc,
        })),
        Err(error) => {
            let res = error.to_string();
            println!("{res:?}");
            if res.contains("code: 11000") {
                return Err(AppError::DuplicateRecord);
            }
            Err(AppError::BadRequest)
        }
    }
}

//Updates one project, does not change material
pub async fn update_project(
    Path(_id): Path<String>,
    State(state): State<AppState>,
    input: ProjectUpdate,
) -> Result<Json<DocResponse<ProjectWithoutMaterials>>, AppError> {
    let oid = parse_oid(_id)?;
    match projects::update_one(&state.db, oid, Json(input)).await {
        Ok(project_doc) => {
            if project_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: project_doc.unwrap(),
            }))
        }
        Err(_error) => {
            let error = _error.kind.to_string();
            if error.contains("name_1") {
                return Err(AppError::DuplicateRecord);
            }
            Err(AppError::BadRequest)
        }
    }
}

pub async fn delete_project(
    Path(_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<DocResponse<DeletedProject>>, AppError> {
    let oid = parse_oid(_id)?;
    match projects::delete_one(&state.db, oid).await {
        Ok(project_doc) => {
            if project_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "success".to_string(),
                data: project_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}

//Insert one material object to a project project
pub async fn insert_project_material(
    Path(_id): Path<String>,
    State(state): State<AppState>,
    Json(input): Json<MaterialWithSysIndicator>,
) -> Result<Json<DocResponse<UpdatedMaterials>>, AppError> {
    let oid = parse_oid(_id)?;
    match projects::insert_material(&state.db, oid, input).await {
        Ok(project_doc) => Ok(Json(DocResponse {
            status: "success".to_string(),
            data: project_doc.unwrap(),
        })),
        Err(_error) => Err(AppError::BadRequest),
    }
}

//Update one material object to a project project
pub async fn update_project_material(
    Path(_id): Path<String>,
    State(state): State<AppState>,
    Json(input): Json<MaterialWithSysIndicator>,
) -> Result<Json<DocResponse<UpdatedMaterials>>, AppError> {
    let oid = parse_oid(_id)?;
    match projects::update_material(&state.db, oid, input).await {
        Ok(project_doc) => Ok(Json(DocResponse {
            status: "success".to_string(),
            data: project_doc.unwrap(),
        })),
        Err(_error) => Err(AppError::BadRequest),
    }
}

//Update one material object to a project project
pub async fn remove_project_material(
    Path(_id): Path<String>,
    State(state): State<AppState>,
    Json(input): Json<MaterialToDelete>,
) -> Result<Json<DocResponse<UpdatedMaterials>>, AppError> {
    let oid = parse_oid(_id)?;
    match projects::remove_material(&state.db, oid, input).await {
        Ok(project_doc) => Ok(Json(DocResponse {
            status: "success".to_string(),
            data: project_doc.unwrap(),
        })),
        Err(_error) => Err(AppError::BadRequest),
    }
}
