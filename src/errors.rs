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
    // InternalServerError,
    // UserDoesNotExist,
    // UserAlreadyExists,
    CannotCreate,
    OidParseError,
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::OidParseError => (StatusCode::BAD_REQUEST, "Cannot parse oid"),
            Self::NotFound => (StatusCode::NOT_FOUND, "Requested resource does not exist"),
            Self::CannotCreate => (StatusCode::BAD_REQUEST, "Could not create resouce"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
