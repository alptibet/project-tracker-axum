use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub enum AppError {
    NoAuth,
    NotAuthorized,
    WrongCredentials,
    InternalServerError,
    UserDoesNotExist,
    UserAlreadyExists,
    EmailAlreadyExists,
    UserNotActive,
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
            Self::UserNotActive => (StatusCode::FORBIDDEN, "Your account is not active"),
            Self::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong user credentials"),
            Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, "This username is already taken"),
            Self::EmailAlreadyExists => (
                StatusCode::BAD_REQUEST,
                "This email address belongs to another user.",
            ),
            Self::NoAuth => (StatusCode::UNAUTHORIZED, "Login to access this resource"),
            Self::NotAuthorized => (
                StatusCode::UNAUTHORIZED,
                "You are not authorized to access this resource",
            ),
        };
        (
            status,
            Json(json!({"status":"Failure", "message": err_msg })),
        )
            .into_response()
    }
}
