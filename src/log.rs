use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, LineWriter, Write};

const SET_NAME: &str = "SET";
const RM_NAME: &str = "RM";

pub enum Operation {
    SET(String, String),
    RM(String),
}

impl Operation {
    pub fn name(op: &Operation) -> &'static str {
        match op {
            Operation::SET(_, _) => SET_NAME,
            Operation::RM(_) => RM_NAME,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LogRecord {
    op: &'static str,
    key: String,
    value: Option<String>,
}

impl LogRecord {
    pub fn from_operation(op: &Operation) -> LogRecord {
        match op {
            Operation::SET(key, value) => LogRecord {
                op: Operation::name(op),
                key: String::from(key),
                value: Some(String::from(value)),
            },
            Operation::RM(key) => LogRecord {
                op: Operation::name(op),
                key: String::from(key),
                value: None,
            },
        }
    }
}

pub struct LogWriter {
    buf_writer: LineWriter<File>,
}

impl LogWriter {
    pub fn new(path: &str) -> Result<LogWriter> {
        let file = File::create(path)?;
        let buf = LineWriter::new(file);
        Ok(LogWriter { buf_writer: buf })
    }

    pub fn write(&mut self, op: &Operation) -> Result<()> {
        let record = LogRecord::from_operation(op);
        let json = LogWriter::log_record(&record)?;
        self.buf_writer.write_all(json.as_bytes())?;
        self.buf_writer.write_all(b"\n")?;
        Ok(())
    }

    fn log_record(record: &LogRecord) -> Result<String> {
        let json = serde_json::to_string(record)?;
        Ok(json)
    }
}
