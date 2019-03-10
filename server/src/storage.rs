use rocksdb;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait StorageLayer: Clone {
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

#[derive(Clone)]
pub struct RocksDbStorageLayer {
    db: Arc<rocksdb::DB>,
}

impl RocksDbStorageLayer {
    pub fn new(path: String) -> RocksDbStorageLayer {
        RocksDbStorageLayer {
            db: Arc::new(rocksdb::DB::open_default(path).unwrap()),
        }
    }
}

impl StorageLayer for RocksDbStorageLayer {
    fn put(&self, key: String, value: String) {
        self.db.put(key.as_bytes(), value.as_bytes()).unwrap();
    }

    fn get(&self, key: &str) -> Option<String> {
        self.db
            .get(key.as_bytes())
            .unwrap()
            .map(|v| String::from_utf8(v.to_vec()).unwrap())
    }
}
