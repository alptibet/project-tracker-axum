use app::run_app;
use dotenv::dotenv;
mod app;
mod appstate;
mod controllers;
mod db;
mod errors;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run_app().await;
}
