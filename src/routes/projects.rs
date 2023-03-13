use crate::controllers::projects;
use crate::errors::AppError;

use crate::models::projects::{ProjectInput, ProjectWithMaterials};
use crate::models::response::{DocResponse, VecResponse};

use crate::utils::parse_oid;
use crate::{appstate::AppState, models::projects::ProjectWithoutMaterials};
use axum::extract::{Json, Path, State};

//Gets all projects without materials data
pub async fn get_all_projects(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<ProjectWithoutMaterials>>, AppError> {
    match projects::get_all(&state.db).await {
        Ok(_projects_doc) => Ok(Json(VecResponse {
            message: "Success".to_string(),
            data: _projects_doc,
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

//Gets one project with materials data
pub async fn get_one_project_with_materials(
    State(state): State<AppState>,
    Path(_id): Path<String>,
) -> Result<Json<DocResponse<ProjectWithMaterials>>, AppError> {
    let oid = parse_oid(_id)?;
    match projects::get_one_with_materials(&state.db, oid).await {
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

//Inserts a project without materials
pub async fn insert_project(
    State(state): State<AppState>,
    input: ProjectInput,
) -> Result<Json<DocResponse<ProjectWithoutMaterials>>, AppError> {
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

// //Updates one project, does not change material
// pub async fn update_project(
//     Path(_id): Path<String>,
//     State(state): State<AppState>,
//     input: ProjectInput,
// ) -> Result<Json<DocResponse<Project>>, AppError> {
//     let oid = parse_oid(_id)?;
//     match projects::update_one(&state.db, oid, Json(input)).await {
//         Ok(_project_doc) => {
//             if _project_doc.is_none() {
//                 return Err(AppError::NotFound);
//             }
//             Ok(Json(DocResponse {
//                 message: "success".to_string(),
//                 data: _project_doc.unwrap(),
//             }))
//         }
//         Err(_error) => {
//             let error = _error.kind.to_string();
//             if error.contains("name_1") {
//                 return Err(AppError::DuplicateRecord);
//             }
//             Err(AppError::BadRequest)
//         }
//     }
// }

// pub async fn delete_project(
//     Path(_id): Path<String>,
//     State(state): State<AppState>,
// ) -> Result<Json<DocResponse<Project>>, AppError> {
//     let oid = parse_oid(_id)?;
//     match projects::delete_one(&state.db, oid).await {
//         Ok(_project_doc) => {
//             if _project_doc.is_none() {
//                 return Err(AppError::NotFound);
//             }
//             Ok(Json(DocResponse {
//                 message: "success".to_string(),
//                 data: _project_doc.unwrap(),
//             }))
//         }
//         Err(_error) => Err(AppError::BadRequest),
//     }
// }
