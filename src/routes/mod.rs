mod auth;
mod contractors;
mod users;
use self::{
    auth::signup,
    contractors::{
        delete_contractor, get_all_contractors, get_one_contractor, insert_contractor,
        update_contractor,
    },
    users::{delete_user, get_all_users, get_one_user},
};
use crate::db::init_db;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use tower_http::trace::TraceLayer;

pub async fn create_routes() -> Router {
    tracing_subscriber::fmt::init();
    let db = init_db().await;
    Router::new()
        .route("/api/v1/users", post(signup))
        .route("/api/v1/contractors", get(get_all_contractors))
        .route("/api/v1/contractors/:id", get(get_one_contractor))
        .route("/api/v1/contractors", post(insert_contractor))
        .route("/api/v1/contractors/:id", delete(delete_contractor))
        .route("/api/v1/contractors/:id", patch(update_contractor))
        .route("/api/v1/users", get(get_all_users))
        .route("/api/v1/users/:id", get(get_one_user))
        .route("/api/v1/users", patch(delete_user))
        .with_state(db)
        .layer(TraceLayer::new_for_http())
}
