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

impl<T: Read + Seek> Read for SeekReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<T: Read + Seek> Seek for SeekReader<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

pub struct SeekWriter<T: Write + Seek> {
    pub pos: u64,
    writer: BufWriter<T>,
}

impl<T: Write + Seek> SeekWriter<T> {
    pub fn new(mut source: T) -> Result<Self> {
        let pos = source.seek(SeekFrom::Current(0))?;
        let writer = BufWriter::new(source);
        Ok(SeekWriter { pos, writer })
    }
}

impl<T: Write + Seek> Write for SeekWriter<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<T: Write + Seek> Seek for SeekWriter<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let pos = self.writer.seek(pos)?;
        self.pos += pos as u64;
        Ok(self.pos)
    }
}
