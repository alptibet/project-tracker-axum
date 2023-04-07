mod auth;
mod contractors;
mod materials;
mod projects;
mod systems;
mod users;
mod views;
use self::{
    auth::{login, logout, signup},
    contractors::create_contractors_routes,
    materials::create_materials_routes,
    projects::create_projects_routes,
    systems::create_systems_routes,
    users::{create_users_routes, get_me, update_me},
    views::create_view_routes,
};

use crate::{
    appstate::AppState,
    controllers::auth::{authenticate_user, authorize_admin},
};
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    middleware,
    routing::{get, post},
    Router,
};

use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub async fn create_routes(appstate: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_origin("http://localhost:5174".parse::<HeaderValue>().unwrap());

    tracing_subscriber::fmt::init();

    let contrators_routes = create_contractors_routes();
    let materials_routes = create_materials_routes();
    let system_routes = create_systems_routes();
    let projects_routes = create_projects_routes();
    let user_routes = create_users_routes();

    Router::new()
        .nest("/api/v1/contractors", contrators_routes)
        .nest("/api/v1/materials", materials_routes)
        .nest("/api/v1/systems", system_routes)
        .nest("/api/v1/projects", projects_routes)
        .nest("/api/v1/users", user_routes)
        .layer(middleware::from_fn(authorize_admin))
        .route("/api/v1/users/me", get(get_me).patch(update_me))
        .route_layer(middleware::from_fn_with_state(
            appstate.clone(),
            authenticate_user,
        ))
        .nest("/", create_view_routes())
        .route("/api/v1/logout", post(logout))
        .route("/api/v1/signup", post(signup))
        .route("/api/v1/login", post(login))
        .with_state(appstate)
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
}
