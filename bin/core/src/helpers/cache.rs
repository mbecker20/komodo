use std::collections::HashMap;

use monitor_types::busy::Busy;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct Cache<T: Clone + Default> {
    cache: RwLock<HashMap<String, T>>,
}

impl<T: Clone + Default> Cache<T> {
    pub async fn get(&self, key: &str) -> Option<T> {
        self.cache.read().await.get(key).cloned()
    }

    // pub async fn get_or_default(&self, key: String) -> T {
    //     let mut cache = self.cache.write().await;
    //     cache.entry(key).or_default().clone()
    // }

    pub async fn get_list(
        &self,
        // filter: Option<impl Fn(&String, &T) -> bool>
    ) -> Vec<T> {
        let cache = self.cache.read().await;
        // match filter {
        //     Some(filter) => cache
        //         .iter()
        //         .filter(|(k, v)| filter(k, v))
        //         .map(|(_, e)| e.clone())
        //         .collect(),
        //     None => cache.iter().map(|(_, e)| e.clone()).collect(),
        // }
        cache.iter().map(|(_, e)| e.clone()).collect()
    }

    pub async fn insert(&self, key: impl Into<String>, val: T) {
        self.cache.write().await.insert(key.into(), val);
    }

    pub async fn update_entry(&self, key: impl Into<String>, handler: impl Fn(&mut T)) {
        let mut cache = self.cache.write().await;
        handler(cache.entry(key.into()).or_default());
    }

    // pub async fn clear(&self) {
    //     self.cache.write().await.clear();
    // }

    pub async fn remove(&self, key: &str) {
        self.cache.write().await.remove(key);
    }
}

impl<T: Clone + Default + Busy> Cache<T> {
    pub async fn busy(&self, id: &str) -> bool {
        match self.get(id).await {
            Some(state) => state.busy(),
            None => false,
        }
    }
}
