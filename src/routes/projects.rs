use crate::controllers::projects;
use crate::errors::AppError;
use crate::models::response::VecResponse;
use crate::{appstate::AppState, models::projects::Project};
use axum::extract::{Json, State};

// pub async fn get_all_projects(
//     State(state): State<AppState>,
// ) -> Result<Json<VecResponse<Project>>, AppError> {
//     match projects::get_all(&state.db).await {
//         Ok(_projects_doc) => Ok(Json(VecResponse {
//             message: "Success".to_string(),
//             data: _projects_doc,
//         })),
//         Err(_error) => Err(AppError::NotFound),
//     }
// }

pub async fn get_all_projects(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<Project>>, AppError> {
    match projects::get_all(&state.db).await {
        Ok(_projects_doc) => Ok(Json(VecResponse {
            message: "Success".to_string(),
            data: _projects_doc,
        })),
        Err(_error) => Err(AppError::DuplicateRecord),
    }
}
