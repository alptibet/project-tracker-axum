mod auth;
mod contractors;
mod projects;
mod users;
use self::{
    auth::{login, logout, signup},
    contractors::{
        delete_contractor, get_all_contractors, get_one_contractor, insert_contractor,
        update_contractor,
    },
    projects::get_all_projects,
    users::{get_all_users, get_me, get_one_user, update_me, update_user},
};
use crate::{
    appstate::AppState,
    controllers::auth::{authenticate_user, authorize_admin},
};
use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;

pub async fn create_routes(appstate: AppState) -> Router {
    tracing_subscriber::fmt::init();
    Router::new()
        .route("/api/v1/projects", get(get_all_projects))
        .route("/api/v1/contractors", get(get_all_contractors))
        .route("/api/v1/contractors/:id", get(get_one_contractor))
        .route("/api/v1/contractors", post(insert_contractor))
        .route("/api/v1/contractors/:id", delete(delete_contractor))
        .route("/api/v1/contractors/:id", patch(update_contractor))
        .route("/api/v1/users", get(get_all_users))
        .route("/api/v1/users/:id", get(get_one_user))
        .route("/api/v1/users/:id", patch(update_user))
        .layer(middleware::from_fn(authorize_admin))
        .route("/api/v1/users/me", get(get_me).patch(update_me))
        .route_layer(middleware::from_fn_with_state(
            appstate.clone(),
            authenticate_user,
        ))
        .route("/api/v1/logout", post(logout))
        .route("/api/v1/signup", post(signup))
        .route("/api/v1/login", post(login))
        .with_state(appstate)
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
}
