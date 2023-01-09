use app::run_app;
use dotenv::dotenv;
mod app;
mod appstate;
mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run_app().await;
}
