use crate::appstate::AppState;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};

#[derive(Template)]
#[template(path = "hello.html")]

struct HelloTemplate<'a> {
    name: &'a str,
}

pub fn create_view_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(render_home))
        .route("/yarr", get(render_yarr))
}

pub async fn render_home() -> impl IntoResponse {
    let hello = HelloTemplate { name: "Alp" };
    match hello.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}
pub async fn render_yarr() -> impl IntoResponse {
    let hello = HelloTemplate { name: "Yarrak" };
    match hello.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}
