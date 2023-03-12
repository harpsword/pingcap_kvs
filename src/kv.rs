use std::collections::HashMap;


/// KvStore is a kvs store
pub struct KvStore {
    storage: HashMap<String, String>,
}

impl KvStore {
    /// Creates a KvStore
    pub fn new() -> Self {
        KvStore {  storage: HashMap::new()}
    }

    /// Set value
    pub fn set(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    /// get value
    pub fn get(&self, key: String) -> Option<String> {
        self.storage.get(&key).cloned()
    }

    /// Remove value
    pub fn remove(&mut self, key: String) {
        self.storage.remove(&key);
    }
}