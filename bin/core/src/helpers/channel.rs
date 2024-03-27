use std::sync::OnceLock;

use monitor_client::entities::update::{Update, UpdateListItem};
use tokio::sync::{broadcast, Mutex};

/// A channel sending (build_id, update_id)
pub fn build_cancel_channel(
) -> &'static BroadcastChannel<(String, Update)> {
  static BUILD_CANCEL_CHANNEL: OnceLock<
    BroadcastChannel<(String, Update)>,
  > = OnceLock::new();
  BUILD_CANCEL_CHANNEL.get_or_init(|| BroadcastChannel::new(100))
}

pub fn update_channel() -> &'static BroadcastChannel<UpdateListItem> {
  static UPDATE_CHANNEL: OnceLock<BroadcastChannel<UpdateListItem>> =
    OnceLock::new();
  UPDATE_CHANNEL.get_or_init(|| BroadcastChannel::new(100))
}

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
