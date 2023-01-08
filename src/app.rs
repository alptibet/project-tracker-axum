use crate::db;
use crate::routes;
use axum::Router;

pub async fn run_app() -> Router {
    let db = db::init_db().await;
    Router::new().merge(
        Router::new().nest(
            "/api/v1",
            Router::new()
                .merge(routes::contractors::create_routes())
                .merge(routes::users::create_routes()),
        ),
    )
}
