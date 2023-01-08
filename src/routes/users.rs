use axum::extract::State;
use axum::response::Html;
use axum::{routing::get, Json, Router};

use crate::appstate::AppState;
use crate::controllers::contractors;
use crate::errors::AppError;
use crate::models::contractors::Contractor;
use crate::models::response::VecResponse;

pub fn create_routes() -> Router {
    Router::new().route("/users", get(handler))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello From Users!</h1>")
}
