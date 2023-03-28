use crate::{appstate::AppState, db::init_db, routes::create_routes};

pub async fn run_app() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server started on {addr}");
    let db = init_db().await;
    let appstate = AppState { db };
    let app = create_routes(appstate).await;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("FAILED TO START SERVER");
}
