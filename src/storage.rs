use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::log::{LogWriter, Operation};
use anyhow::Result;

pub struct KVStorage {
    store: HashMap<String, String>,
    log_writer: LogWriter
}

impl KVStorage {
    pub fn new() -> Result<KVStorage> {
        let writer = LogWriter::new("log")?;
        Ok(KVStorage {
            store: HashMap::new(),
            log_writer: writer
        })
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key.clone(), value.clone());
        self.log_writer.write(&Operation::SET(key, value.clone()))?;
        Ok(())
    }

    pub fn delete(&mut self, key: &str) -> Result<()> {
        self.store.remove(key);
        self.log_writer.write(&Operation::RM(key.to_string()))?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
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
