mod contractors;
use crate::db::init_db;
use axum::{routing::get, Router};

use self::contractors::{get_contractors, get_one_contractor};

pub async fn create_routes() -> Router {
    let db = init_db().await;
    Router::new()
        .route("/api/v1/contractors", get(get_contractors))
        .route("/api/v1/contractors/:id", get(get_one_contractor))
        .with_state(db)
}
