use std::{collections::HashMap, hash::Hash};

pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        _ = self.store.insert(key, value)
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).map(|v| v.clone())
    }

    pub fn remove(&mut self, key: String) {
        _ = self.store.remove(&key)
    }
}
