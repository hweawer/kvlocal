use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::{fmt, result};

pub struct KVStorage<T> {
    store: HashMap<String, T>,
}

impl<T> KVStorage<T> {
    pub fn new() -> KVStorage<T> {
        KVStorage {
            store: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: T) {
        self.store.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) -> result::Result<(), StorageError> {
        match self.store.remove(key) {
            Some(_) => result::Result::Ok(()),
            None => result::Result::Err(StorageError(String::from(
                "You are trying to delete by key which is not present",
            ))),
        }
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.store.get(key)
    }
}

#[derive(Debug)]
pub struct StorageError(String);
impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Storage error: {}", self.0)
    }
}
impl Error for StorageError {}
