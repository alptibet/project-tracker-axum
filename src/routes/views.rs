use crate::{appstate::AppState, controllers::auth::authenticate_user, models::users::ValidUser};
use askama::Template;
use axum::{middleware, routing::get, Extension, Router};

pub fn create_view_routes(appstate: AppState) -> Router<AppState> {
    Router::new()
        .route("/overview", get(render_overview))
        .route_layer(middleware::from_fn_with_state(appstate, authenticate_user))
        .route("/signup", get(render_signup))
        .route("/", get(render_home))
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    title: &'a str,
}

pub async fn render_home() -> HomeTemplate<'static> {
    HomeTemplate { title: "LOGIN" }
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate<'a> {
    title: &'a str,
}

pub async fn render_signup() -> SignupTemplate<'static> {
    SignupTemplate { title: "SIGNUP" }
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
        title: "PROJECTS",
        name: user.name,
        surname: user.surname,
    }
}
