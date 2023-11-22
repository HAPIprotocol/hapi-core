use {
    axum::{response::IntoResponse, routing::post, Router, Server},
    reqwest::StatusCode,
    std::net::SocketAddr,
};

async fn webhook_handler(body: String) -> impl IntoResponse {
    println!("Received webhook: {}\n", body);

    (StatusCode::OK, "Received webhook")
}

#[tokio::test]
async fn run_simple_listener() {
    let app = Router::new()
        .route("/", post(webhook_handler))
        .into_make_service();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app)
        .await
        .expect("Failed to start server");
}
