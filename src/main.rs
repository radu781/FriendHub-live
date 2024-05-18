use axum::routing::get;
use chat::handler::namespace::on_chat_ns_joined;
use log::debug;
use notifications::{notif::VotingServer, notif::VotingService};
use socketioxide::SocketIo;
use tonic::transport::Server;

mod chat;
mod notifications;
mod utils;

async fn setup_socketio() {
    let (layer, io) = SocketIo::new_layer();
    io.ns("/live/chat", on_chat_ns_joined);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind");
    axum::serve(listener, app).await.expect("serve");
}

async fn setup_grpc() {
    let voting_service = VotingService::default();
    Server::builder()
        .add_service(VotingServer::new(voting_service))
        .serve("0.0.0.0:3001".parse().expect("ip parse failed"))
        .await
        .expect("grpc server failed");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::join!(setup_socketio(), setup_grpc());
    Ok(())
}
