use anyhow::Result;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = PathBuf::from(path);
    kvlocal::run(path)
}
