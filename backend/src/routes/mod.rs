mod auth;
mod contractors;
mod materials;
mod projects;
mod systems;
mod users;
use self::{
    auth::{login, logout, signup},
    contractors::{
        delete_contractor, get_all_contractors, get_one_contractor, insert_contractor,
        update_contractor,
    },
    materials::{
        delete_material, get_all_materials, get_one_material, insert_material, update_material,
    },
    projects::{
        delete_project, get_all_projects, get_one_project, get_one_project_with_materials,
        insert_project, insert_project_material, remove_project_material, update_project,
        update_project_material,
    },
    systems::{delete_system, get_all_systems, get_one_system, insert_system, update_system},
    users::{get_all_users, get_me, get_one_user, update_me, update_user},
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
    routing::{delete, get, patch, post},
    Router,
};

use tower_cookies::CookieManagerLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub async fn create_routes(appstate: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_origin("http://localhost:5174".parse::<HeaderValue>().unwrap());

    tracing_subscriber::fmt::init();

    Router::new()
        .route("/api/v1/materials", get(get_all_materials))
        .route("/api/v1/materials/:id", get(get_one_material))
        .route("/api/v1/materials", post(insert_material))
        .route("/api/v1/materials/:id", delete(delete_material))
        .route("/api/v1/materials/:id", patch(update_material))
        .route("/api/v1/systems", get(get_all_systems))
        .route("/api/v1/systems/:id", get(get_one_system))
        .route("/api/v1/systems", post(insert_system))
        .route("/api/v1/systems/:id", delete(delete_system))
        .route("/api/v1/systems/:id", patch(update_system))
        .route("/api/v1/projects/:id", delete(delete_project))
        .route(
            "/api/v1/projects/removematerial/:id",
            patch(remove_project_material),
        )
        .route(
            "/api/v1/projects/editmaterial/:id",
            patch(update_project_material),
        )
        .route(
            "/api/v1/projects/addmaterial/:id",
            patch(insert_project_material),
        )
        .route("/api/v1/projects/:id", patch(update_project))
        .route("/api/v1/projects", post(insert_project))
        .route(
            "/api/v1/projects/detailed/:id",
            get(get_one_project_with_materials),
        )
        .route("/api/v1/projects/:id", get(get_one_project))
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
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
}
