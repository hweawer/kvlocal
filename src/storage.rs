use crate::log::{LogRecord, LogWriter, Operation};
use crate::seek::{SeekReader, SeekWriter};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::{create_dir_all, read_dir, DirEntry, File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::ops::Add;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref LOG_NAME_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

type Generation = u64;

pub struct KVStorage {
    index: HashMap<String, IndexValue>,
    writer: SeekWriter<File>,
    readers: HashMap<Generation, SeekReader<File>>,
    store: HashMap<String, String>,
}

struct IndexValue {
    gen: Generation,
    offset: u64,
    len: u64,
}

struct LogName(String);

impl From<&Generation> for LogName {
    fn from(num: &Generation) -> Self {
        LogName(num.to_string().add(".log"))
    }
}

impl LogName {
    fn is_log(path: impl Into<PathBuf> + Copy) -> bool {
        let path = path
            .into()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        LOG_NAME_REGEX.is_match(&path)
    }
}

impl KVStorage {
    pub fn new(path: PathBuf) -> Result<KVStorage> {
        create_dir_all(&path)?;
        let generations = KVStorage::sorted_generations(&path)?;
        let mut readers: HashMap<Generation, SeekReader<File>> = HashMap::new();
        let index: HashMap<String, IndexValue> = HashMap::new();
        for &gen in &generations {
            let mut reader: SeekReader<File> =
                SeekReader::new(File::open(path.join(LogName::from(&gen).0))?)?;
            readers.insert(gen, reader);
        }
        let gen = generations.last().unwrap_or(&0) + 1;
        let writer = KVStorage::new_log_file(gen, &path, &mut readers)?;
        Ok(KVStorage {
            index,
            writer,
            readers,
            store: HashMap::new(),
        })
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key.clone(), value.clone());
        //self.writer.write(&Operation::SET(key, value.clone()))?;
        Ok(())
    }

    pub fn delete(&mut self, key: &str) -> Result<()> {
        self.store.remove(key);
        //self.writer.write(&Operation::RM(key.to_string()))?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    fn new_log_file(
        gen: Generation,
        dir: &Path,
        readers: &mut HashMap<Generation, SeekReader<File>>,
    ) -> Result<SeekWriter<File>> {
        let new_file_path = dir.join(LogName::from(&gen).0);
        let writer = SeekWriter::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&new_file_path)?,
        )?;
        let reader = SeekReader::new(File::open(&new_file_path)?)?;
        readers.insert(gen, reader);
        Ok(writer)
    }

    fn sorted_generations(path: &PathBuf) -> Result<Vec<Generation>> {
        let mut generations: Vec<Generation> = read_dir(path)?
            .flat_map(|dir| dir.map(|dir| dir.path()))
            .filter(|path| LogName::is_log(path))
            .flat_map(|path| {
                path.file_name()
                    .and_then(|s| s.to_str())
                    .map(str::parse::<Generation>)
            })
            .flatten()
            .collect();
        generations.sort_unstable();
        Ok(generations)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_from_log_name() {
        let num: u64 = 11239840;
        let log_name = LogName::from(&num);
        assert_eq!(log_name.0, "11239840.log")
    }

    #[test]
    fn test_is_log() {
        let x = Path::new("11239840.log");
        assert!(LogName::is_log(x));
    }
}
