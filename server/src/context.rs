use tokio::sync::broadcast::{self, Sender};
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};

use crate::domain::user::UserMessage;

#[derive(Clone, Debug)]
pub struct MyContext {
    pub user_message_tx: Sender<UserMessage>,
}

impl MyContext {
    pub fn new() -> Self {
        Self {
            user_message_tx: broadcast::channel(50).0,
        }
    }

    pub fn stream_from<T: 'static + Clone + Send>(tx: &Sender<T>) -> impl Stream<Item = T> {
        let txs = tx.subscribe();
        let stream = BroadcastStream::new(txs);

        stream.filter_map(|b| b.ok())
    }
}
