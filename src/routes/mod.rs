mod contractors;
use crate::db::init_db;
use axum::{routing::get, Router};

use self::contractors::api_hello;

pub async fn create_routes() -> Router {
    let db = init_db().await;
    Router::new()
        .route("/api/v1/hello", get(api_hello))
        .with_state(db)
}
