use crate::appstate::AppState;
use crate::controllers::contractors;
use crate::errors::AppError;
use crate::models::contractors::{Contractor, ContractorInput};
use crate::models::response::{DocResponse, MessageResponse, VecResponse};
use crate::utils::parse_oid;
use axum::extract::{Json, Path, State};

pub async fn get_all_contractors(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<Contractor>>, AppError> {
    match contractors::get_all(&state.db).await {
        Ok(_contractors_doc) => Ok(Json(VecResponse {
            status: "Success".to_string(),
            data: _contractors_doc,
        })),
        Err(_error) => Err(AppError::NotFound),
    }
}

pub async fn get_one_contractor(
    Path(_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<DocResponse<Contractor>>, AppError> {
    let oid = parse_oid(_id)?;
    match contractors::get_one(&state.db, oid).await {
        Ok(_contractor_doc) => {
            if _contractor_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "Success".to_string(),
                data: _contractor_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::NotFound),
    }
}

pub async fn insert_contractor(
    State(state): State<AppState>,
    input: ContractorInput,
) -> Result<Json<DocResponse<Contractor>>, AppError> {
    match contractors::insert_one(&state.db, Json(input)).await {
        Ok(_contractor_doc) => Ok(Json(DocResponse {
            status: "Success".to_string(),
            data: _contractor_doc,
        })),
        Err(_error) => {
            let res = _error.to_string();
            if res.contains("code: 11000") {
                return Err(AppError::DuplicateRecord);
            }
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn delete_contractor(
    Path(_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<MessageResponse>, AppError> {
    let oid = parse_oid(_id)?;
    match contractors::delete_one(&state.db, oid).await {
        Ok(_contractor_doc) => {
            if _contractor_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(MessageResponse {
                status: "Success".to_string(),
            }))
        }
        Err(_error) => Err(AppError::InternalServerError),
    }
}

pub async fn update_contractor(
    Path(_id): Path<String>,
    State(state): State<AppState>,
    input: ContractorInput,
) -> Result<Json<DocResponse<Contractor>>, AppError> {
    let oid = parse_oid(_id)?;
    match contractors::update_one(&state.db, oid, Json(input)).await {
        Ok(_contractor_doc) => {
            if _contractor_doc.is_none() {
                return Err(AppError::NotFound);
            }
            Ok(Json(DocResponse {
                status: "Success".to_string(),
                data: _contractor_doc.unwrap(),
            }))
        }
        Err(_error) => {
            let res = _error.to_string();
            if res.contains("E11000") {
                return Err(AppError::DuplicateRecord);
            }
            Err(AppError::BadRequest)
        }
    }
}
