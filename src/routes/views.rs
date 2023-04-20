use crate::{appstate::AppState, controllers::auth::authenticate_user, models::users::ValidUser};
use askama::Template;
use axum::{middleware, routing::get, Extension, Router};

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
    name: String,
    surname: String,
}

pub async fn render_overview(Extension(user): Extension<ValidUser>) -> OverviewTemplate<'static> {
    OverviewTemplate {
        title: "Testing",
        name: user.name,
        surname: user.surname,
    }
}
