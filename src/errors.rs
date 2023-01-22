use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    // InvalidToken,
    // WrongCredentials,
    // MissingCredential,
    // TokenCreation,
    InternalServerError,
    // UserDoesNotExist,
    // UserAlreadyExists,
    DuplicateRecord,
    OidParseError,
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::OidParseError => (StatusCode::BAD_REQUEST, "Cannot parse oid"),
            Self::NotFound => (StatusCode::NOT_FOUND, "Requested resource does not exist"),
            Self::DuplicateRecord => (StatusCode::BAD_REQUEST, "Duplicate record found"),
            Self::InternalServerError => (StatusCode::BAD_REQUEST, "Something went wrong"),
        };
        (
            status,
            Json(json!({"status":"Failure", "message": err_msg })),
        )
            .into_response()
    }
}