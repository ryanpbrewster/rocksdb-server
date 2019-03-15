use std::collections::{Bound};
use std::collections::BTreeMap;
use std::ops::RangeBounds;
use std::sync::{Arc, Mutex};

use futures::sync::mpsc::UnboundedReceiver;
use rocksdb;
use rocksdb::{IteratorMode};

use crate::{KvError, KvResult};

pub trait StorageLayer: Clone {
    fn put(&self, key: String, value: String) -> KvResult<()>;
    fn get(&self, key: &str) -> KvResult<Option<String>>;
    fn scan(&self, start: Bound<String>, end: Bound<String>) -> UnboundedReceiver<(String, String)>;
}

impl From<rocksdb::Error> for KvError {
    fn from(err: rocksdb::Error) -> Self {
        KvError::Unknown(Box::new(err))
    }
}

#[derive(Clone, Debug, Default)]
pub struct InMemoryStorageLayer {
    data: Arc<Mutex<BTreeMap<String, String>>>,
}

impl StorageLayer for InMemoryStorageLayer {
    fn put(&self, key: String, value: String) -> KvResult<()> {
        self.data.lock().unwrap().insert(key, value);
        Ok(())
    }

    fn get(&self, key: &str) -> KvResult<Option<String>> {
        Ok(self.data.lock().unwrap().get(key).cloned())
    }

    fn scan(&self, start: Bound<String>, end: Bound<String>) -> UnboundedReceiver<(String, String)> {
        let (sender, receiver) = futures::sync::mpsc::unbounded();
        let db = self.data.clone();
        println!("starting scan");
        ::std::thread::spawn(move || {
            for (k, v) in db.lock().unwrap().range((start, end)) {
                sender.unbounded_send((k.to_owned(), v.to_owned())).unwrap();
            }
            println!("done scanning");
        });
        receiver
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

    fn scan(&self, start: Bound<String>, end: Bound<String>) -> UnboundedReceiver<(String, String)> {
        let (sender, receiver) = futures::sync::mpsc::unbounded();
        let db = self.db.clone();
        println!("starting scan");
        ::std::thread::spawn(move || {
            let bounds = (start, end);
            for (k, v) in db.iterator(IteratorMode::Start) {
                let key = String::from_utf8(k.to_vec()).unwrap();
                // TODO: optimize this scan by seeking to the start and quitting once we're past the end
                if bounds.contains(&key) {
                    sender.unbounded_send((key, String::from_utf8(v.to_vec()).unwrap())).unwrap();
                }
            }
            println!("done scanning");
        });
        receiver
    }
}
