use dotenv::dotenv;

mod app;
mod appstate;
mod controllers;
mod db;
mod errors;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    let app = app::run_app().await;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("FAILED TO START SERVER");
}
