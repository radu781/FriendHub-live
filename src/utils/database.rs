use database::{DBConnection, Message};
use tokio::runtime::Runtime;

pub(crate) struct MessageDAO {}

impl MessageDAO {
    pub(crate) fn insert(message: Message) {
        tokio::task::block_in_place(|| {
            let handle = tokio::runtime::Handle::current();

            handle.block_on(async {
                let mut db = DBConnection::new().await;
                db.insert::<Message>(&message).await;
            })
        })
    }
}
