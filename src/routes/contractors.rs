use crate::appstate::AppState;
use crate::controllers::contractors;
use crate::errors::AppError;
use crate::models::contractors::Contractor;
use crate::models::response::{DocResponse, VecResponse};
use axum::{extract::Path, extract::State, Json};
use mongodb::bson::oid::ObjectId;

pub async fn get_contractors(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<Contractor>>, AppError> {
    match contractors::get_all(&state.db).await {
        Ok(_contractors_doc) => Ok(Json(VecResponse {
            message: "Success".to_string(),
            data: _contractors_doc,
        })),
        Err(_error) => Err(AppError::NotFound),
    }
}

pub async fn get_one_contractor(
    Path(_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<DocResponse<Contractor>>, AppError> {
    let oid = ObjectId::parse_str(_id);
    if oid.is_err() {
        return Err(AppError::OidParseError);
    }
    match contractors::get_one(&state.db, oid.unwrap()).await {
        Ok(_contractor_doc) => {
            if _contractor_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                message: "Success".to_string(),
                data: _contractor_doc.unwrap(),
            }))
        },
        Err(_error) => Err(AppError::NotFound),
    }
}
