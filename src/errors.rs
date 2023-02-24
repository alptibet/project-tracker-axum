use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotAuthorized,
    WrongCredentials,
    InternalServerError,
    UserDoesNotExist,
    UserAlreadyExists,
    DuplicateRecord,
    BadRequest,
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
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Unable to complete request"),
            Self::UserDoesNotExist => (StatusCode::BAD_REQUEST, "User does not exist"),
            Self::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong user credentials"),
            Self::UserAlreadyExists => (StatusCode::UNAUTHORIZED, "This user already exists"),
            Self::NotAuthorized => (
                StatusCode::UNAUTHORIZED,
                "You are either logged out or not authorized to view this resource.",
            ),
        };
        (
            status,
            Json(json!({"status":"Failure", "message": err_msg })),
        )
            .into_response()
    }
}
