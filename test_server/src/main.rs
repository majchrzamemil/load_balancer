use axum::{routing::get, Router};
use std::env;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").expect("Missing PORT env");
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                println!("health");
                ""
            }),
        )
        .route(
            "/test",
            get(|| async {
                println!("test endpoint");
                "test"
            }),
        );
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
