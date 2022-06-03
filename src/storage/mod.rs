use anyhow::Result;

pub trait StorageEngine {
    fn get(&mut self, k: String) -> Result<Option<String>>;
    fn remove(&mut self, k: String) -> Result<()>;
    fn set(&mut self, k: String, v: String) -> Result<()>;
}

pub mod kvlocal;
pub mod sled;
