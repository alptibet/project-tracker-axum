use crate::appstate::AppState;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    title: &'a str,
}

pub fn create_view_routes() -> Router<AppState> {
    Router::new().route("/", get(render_home))
}

pub async fn render_home() -> impl IntoResponse {
    let home = HomeTemplate {
        title: "Project Tracker",
    };
    match home.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}
