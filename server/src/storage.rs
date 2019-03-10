use rocksdb;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::{KvError, KvResult};

pub trait StorageLayer: Clone {
    fn put(&self, key: String, value: String) -> KvResult<()>;
    fn get(&self, key: &str) -> KvResult<Option<String>>;
}

impl From<rocksdb::Error> for KvError {
    fn from(err: rocksdb::Error) -> Self {
        KvError::Unknown(Box::new(err))
    }
}

#[derive(Clone, Debug, Default)]
pub struct InMemoryStorageLayer {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl StorageLayer for InMemoryStorageLayer {
    fn put(&self, key: String, value: String) -> KvResult<()> {
        self.data.lock().unwrap().insert(key, value);
        Ok(())
    }

    fn get(&self, key: &str) -> KvResult<Option<String>> {
        Ok(self.data.lock().unwrap().get(key).cloned())
    }
}

#[derive(Clone)]
pub struct RocksDbStorageLayer {
    db: Arc<rocksdb::DB>,
}

impl RocksDbStorageLayer {
    pub fn new(path: String) -> KvResult<RocksDbStorageLayer> {
        Ok(RocksDbStorageLayer {
            db: Arc::new(rocksdb::DB::open_default(path)?),
        })
    }
}

impl StorageLayer for RocksDbStorageLayer {
    fn put(&self, key: String, value: String) -> KvResult<()> {
        self.db.put(key.as_bytes(), value.as_bytes())?;
        Ok(())
    }

    fn get(&self, key: &str) -> KvResult<Option<String>> {
        Ok(self
            .db
            .get(key.as_bytes())?
            .map(|v| String::from_utf8(v.to_vec()).unwrap()))
    }
}
