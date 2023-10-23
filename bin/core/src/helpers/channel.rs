use tokio::sync::{broadcast, Mutex};

pub struct BroadcastChannel<T> {
  pub sender: Mutex<broadcast::Sender<T>>,
  pub receiver: broadcast::Receiver<T>,
}

impl<T: Clone> BroadcastChannel<T> {
  pub fn new(capacity: usize) -> BroadcastChannel<T> {
    let (sender, receiver) = broadcast::channel(capacity);
    BroadcastChannel {
      sender: sender.into(),
      receiver,
    }
  }
}
