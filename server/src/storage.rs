use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait StorageLayer: Clone + Sized {
    fn put(&self, key: String, value: String);
    fn get(&self, key: &str) -> Option<String>;
}

#[derive(Clone, Debug, Default)]
pub struct InMemoryStorageLayer {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl StorageLayer for InMemoryStorageLayer {
    fn put(&self, key: String, value: String) {
        self.data.lock().unwrap().insert(key, value);
    }

    fn get(&self, key: &str) -> Option<String> {
        self.data.lock().unwrap().get(key).cloned()
    }
}
