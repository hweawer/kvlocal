use anyhow::Result;
use std::env;
use std::io::{BufReader, BufWriter};
use std::net::TcpListener;
use std::path::PathBuf;
use serde::Deserialize;
use serde_json::Deserializer;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = PathBuf::from(path);
    kvlocal::run(path)
}
