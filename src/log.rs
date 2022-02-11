use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Operation {
    Set(String, String),
    Rm(String),
}

#[derive(Serialize, Deserialize)]
pub struct LogRecord {
    op: &'static str,
    key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}
