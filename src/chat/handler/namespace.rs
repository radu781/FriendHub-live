use socketioxide::extract::SocketRef;

use super::room::{on_chat_connected, on_chat_message_received};

#[allow(clippy::needless_pass_by_value)]
pub fn on_chat_ns_joined(s: SocketRef) {
    s.on("connection", on_chat_connected);
    s.on("chat-message", on_chat_message_received);
}

#[allow(clippy::needless_pass_by_value)]
pub fn on_notifications_ns_joined(s: SocketRef) {}
