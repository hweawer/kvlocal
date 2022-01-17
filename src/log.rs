use std::fs::File;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;
use std::io::{BufWriter, Write};

const SET_NAME: &str = "SET";
const RM_NAME: &str = "RM";

pub enum Operation {
    SET(String, String),
    RM(String, String)
}

impl Operation {
    pub fn name(op: &Operation) -> &str {
        match op {
            Operation::SET(_,_) => SET_NAME,
            Operation::RM(_,_) => RM_NAME
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LogRecord {
    op: &'static str,
    key: String,
    value: String
}

impl LogRecord {
    pub fn from_operation(op: &Operation) -> LogRecord {
        match op {
            Operation::SET(key, value) | Operation::RM(key, value) =>
                LogRecord { op: Operation::name(op), key: String::from(key), value: String::from(value) }
        }
    }
}

pub struct LogWriter {
    file: File,
    buf_writer: BufWriter<File>
}

impl LogWriter {
    pub fn new(path: String) -> Result<LogWriter> {
        let file = File::open(path)?;
        let buf = BufWriter::new(&file)?;
        Ok(LogWriter {
            file,
            buf_writer: buf
        })
    }

    pub fn write(&self, op: &Operation) -> Result<()> {
        let record = LogRecord::from_operation(op);
        let json = LogWriter::log_record(&record)?;
        serde_json::to_writer(bw, &json)?;
        Ok(())
    }

    fn log_record(record: &LogRecord) -> Result<String> {
        let json = serde_json::to_string(record)?;
        Ok(json)
    }
}