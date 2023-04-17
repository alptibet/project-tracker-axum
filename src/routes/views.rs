use crate::appstate::AppState;
use askama::Template;
use axum::{routing::get, Router};

pub fn create_view_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(render_home))
        .route("/overview", get(render_overview))
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
