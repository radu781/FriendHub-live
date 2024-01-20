use crate::chat::Message;
use log::error;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};

#[allow(clippy::needless_pass_by_value)]
pub fn on_chat_connected(s: SocketRef, data: Data<Value>) {
    println!("connected");
    match &serde_json::from_value::<Vec<String>>(data.0) {
        Ok(rooms) => {
            // TODO: do I need this?
            s.extensions.insert(rooms.clone());
            for room_id in rooms {
                s.join(room_id.clone()).ok();
            }
        }
        Err(e) => {
            error!(
                "message received on `connection` is not a `Vec<String>` type, error: {}",
                e.to_string()
            );
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn on_chat_message_received(s: SocketRef, data: Data<Value>) {
    match &serde_json::from_value::<Message>(data.0) {
        Ok(msg) => {
            if let Some(room_name) = msg.room_name() {
                s.join(room_name.clone()).ok();
                s.to(room_name).emit("chat-message", &msg.text).ok();
            } else {
                error!("room name is `None`");
            }
        }
        Err(e) => {
            error!(
                "message received on `chat-message` is not a `Message` type, error: {}",
                e.to_string()
            );
        }
    };
}
