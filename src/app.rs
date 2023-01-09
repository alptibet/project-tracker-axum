use crate::routes::create_routes;

pub async fn run_app() {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    let app = create_routes().await;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("FAILED TO START SERVER");
}
