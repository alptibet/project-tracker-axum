use axum::{routing::get, Router};
use dotenv::dotenv;

mod appstate;
mod controllers;
mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = db::init_db().await;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route(
            "/api/v1/contractors",
            get(routes::contractors::get_contractors).with_state(db),
        );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("FAILED TO START SERVER");
}
