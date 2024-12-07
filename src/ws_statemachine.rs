use std::collections::HashSet;

use tokio::sync::broadcast::{self, error::SendError};

#[derive(Clone, Debug)]
pub struct SocketContainer {
    pub channel_id: i32,
    pub tx: broadcast::Sender<String>,
    pub users: HashSet<String>
}
impl SocketContainer {
    pub fn contains(&self, u: String) -> bool {
        self.users.contains(&u)
    }

    pub fn add(&mut self, u: String) {
        self.users.insert(u);
    }

    pub fn broadcast(&mut self, msg: String) -> Result<usize, SendError<String>> {
        self.tx.send(msg)
    }
}
