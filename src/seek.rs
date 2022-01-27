use anyhow::Result;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

pub struct SeekReader<T: Read + Seek> {
    pos: u64,
    reader: BufReader<T>,
}

impl<T: Read + Seek> SeekReader<T> {
    pub fn new(mut source: T) -> Result<Self> {
        let pos = source.seek(SeekFrom::Current(0))?;
        let reader = BufReader::new(source);
        Ok(SeekReader { pos, reader })
    }
}

pub struct SeekWriter<T: Write + Seek> {
    pos: u64,
    writer: BufWriter<T>,
}

impl<T: Write + Seek> SeekWriter<T> {
    pub fn new(mut source: T) -> Result<Self> {
        let pos = source.seek(SeekFrom::Current(0))?;
        let writer = BufWriter::new(source);
        Ok(SeekWriter { pos, writer })
    }
}
