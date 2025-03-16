use axum::{routing::get, routing::post, Router,body::{Bytes, Body}, http::{Method, HeaderMap, }};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/log", post(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(body: Bytes) {
    println!("{:?}", body);
}

