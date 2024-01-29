use axum::routing::get;
use chat::handler::namespace::on_chat_ns_joined;
use socketioxide::SocketIo;

mod chat;
mod notifications;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::new_layer();
    io.ns("/live/chat", on_chat_ns_joined);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind");
    axum::serve(listener, app).await.expect("serve");

    Ok(())
}
