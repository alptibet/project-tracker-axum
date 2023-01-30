use crate::appstate::AppState;
use crate::controllers::users;
use crate::errors::AppError;
use crate::models::users::{User, UserInput};
use crate::models::response::{DocResponse, MessageResponse, VecResponse};
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
