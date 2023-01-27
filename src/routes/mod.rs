mod contractors;
use crate::db::init_db;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use tower_http::trace::TraceLayer;

use self::contractors::{
    delete_contractor, get_all_contractors, get_one_contractor, insert_contractor,
    update_contractor,
};

pub async fn create_routes() -> Router {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let db = init_db().await;
    Router::new()
        .route("/api/v1/contractors", get(get_all_contractors))
        .route("/api/v1/contractors/:id", get(get_one_contractor))
        .route("/api/v1/contractors", post(insert_contractor))
        .route("/api/v1/contractors/:id", delete(delete_contractor))
        .route("/api/v1/contractors/:id", patch(update_contractor))
        .layer(TraceLayer::new_for_http())
        .with_state(db)
}
