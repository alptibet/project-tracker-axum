use crate::appstate::AppState;
use crate::controllers::auth::authenticate_user;
use askama::Template;
use axum::middleware;
use axum::{routing::get, Router};

pub fn create_view_routes(appstate: AppState) -> Router<AppState> {
    Router::new()
        .route("/overview", get(render_overview))
        .route_layer(middleware::from_fn_with_state(
            appstate.clone(),
            authenticate_user,
        ))
        .route("/", get(render_home))
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    title: &'a str,
}

pub async fn render_home() -> HomeTemplate<'static> {
    HomeTemplate {
        title: "Project Tracker",
    }
}

#[derive(Template)]
#[template(path = "overview.html")]
pub struct OverviewTemplate<'a> {
    title: &'a str,
}

pub async fn render_overview() -> OverviewTemplate<'static> {
    OverviewTemplate { title: "Testing" }
}
