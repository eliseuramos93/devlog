use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { "The app is healthy!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind to port 3000");
    axum::serve(listener, app).await.expect("Failed to serve");
}
