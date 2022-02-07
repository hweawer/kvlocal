use crate::log::Operation;
use crate::seek::{SeekReader, SeekWriter};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Deserializer;
use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;
use std::fs::{create_dir_all, read_dir, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::num::ParseIntError;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::string::String;

lazy_static! {
    static ref LOG_NAME_REGEX: Regex = Regex::new(r"\d+.log").unwrap();
}

type Generation = u64;

pub struct KVStorage {
    gen: Generation,
    index: BTreeMap<String, IndexValue>,
    writer: SeekWriter<File>,
    readers: HashMap<Generation, SeekReader<File>>,
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
        let mut index: BTreeMap<String, IndexValue> = BTreeMap::new();
        for &gen in &generations {
            let mut reader: SeekReader<File> =
                SeekReader::new(File::open(path.join(LogName::from(&gen).0))?)?;
            KVStorage::load_log(gen, &mut index, &mut reader)?;
            readers.insert(gen, reader);
        }
        let gen = generations.last().unwrap_or(&0) + 1;
        let writer = KVStorage::new_log_file(gen, &path, &mut readers)?;
        Ok(KVStorage {
            gen,
            index,
            writer,
            readers,
        })
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<()> {
        let op = Operation::Set(key, value);
        let pos = self.writer.pos;
        serde_json::to_writer(&mut self.writer, &op)?;
        self.writer.flush()?;
        let value = IndexValue {
            gen: self.gen,
            offset: pos,
            len: self.writer.pos - pos,
        };
        if let Operation::Set(key, ..) = op {
            self.index.insert(key, value);
        }
        Ok(())
    }

    pub fn delete(&mut self, key: String) -> Result<()> {
        let op = Operation::Rm(key);
        serde_json::to_writer(&mut self.writer, &op)?;
        let _ = &self.writer.flush();
        if let Operation::Rm(key) = op {
            let _ = &self.index.remove(&key);
        }
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if self.index.contains_key(&key) {
            let value = self.index.get(&key).unwrap();
            let reader = self.readers.get_mut(&value.gen).unwrap();
            reader.seek(SeekFrom::Start(value.offset))?;
            let operation_border = reader.take(value.len);
            if let Operation::Set(_key, val) = serde_json::from_reader(operation_border)? {
                Ok(Some(val))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
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

    fn sorted_generations(path: &Path) -> Result<Vec<Generation>> {
        let mut generations: Vec<Generation> = read_dir(path)?
            .flat_map(|dir| dir.map(|dir| dir.path()))
            .filter(|path| LogName::is_log(path))
            .flat_map(|path| {
                path.file_name()
                    .and_then(|s| s.to_str().map(|x| x.trim_end_matches(".log")))
                    .map(str::parse::<Generation>)
            })
            .collect::<Result<Vec<Generation>, ParseIntError>>()?;
        generations.sort_unstable();
        Ok(generations)
    }

    fn load_log(
        gen: Generation,
        index: &mut BTreeMap<String, IndexValue>,
        reader: &mut SeekReader<File>,
    ) -> Result<()> {
        // 1. Read record
        // 2. If remove ->  remove from index. continue
        // 3. Create IndexValue.
        // 4. Insert by index key
        let mut pos = reader.seek(SeekFrom::Start(0))?;
        let mut stream = Deserializer::from_reader(reader).into_iter::<Operation>();
        while let Some(record) = stream.next() {
            let new_pos = stream.byte_offset() as u64;
            match record? {
                Operation::Set(key, _) => {
                    let index_value = IndexValue {
                        gen,
                        offset: pos,
                        len: new_pos - pos,
                    };
                    index.insert(key, index_value);
                }
                Operation::Rm(key) => {
                    index.remove(&key);
                }
            }
            pos = new_pos;
        }
        Ok(())
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
