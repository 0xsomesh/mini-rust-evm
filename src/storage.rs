use std::collections::HashMap;

pub struct Storage {
    storage: HashMap<[u8; 32], [u8; 32]>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: [u8; 32], value: [u8; 32]) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: &[u8; 32]) -> [u8; 32] {
        *self.storage.get(key).unwrap_or(&[0u8; 32])
    }

    pub fn delete(&mut self, key: &[u8; 32]) {
        self.storage.remove(key);
    }
}