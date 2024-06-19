use serde::{Deserialize, Serialize};
use axum::{
    routing,
    Router,
    Json
};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String
}

async fn json_response() -> Json<ApiResponse> {
    let response = ApiResponse {
        message: "Hello, world!".to_string(),
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(json_response));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
