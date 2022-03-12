use anyhow::Result;
use std::{env, fs};
use std::env::current_dir;
use std::io::{BufReader, BufWriter};
use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;
use serde::Deserialize;
use serde_json::Deserializer;
use structopt::clap::arg_enum;
use structopt::StructOpt;
use log::{debug, error, log_enabled, info, Level, LevelFilter};

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvlocal")]
struct Opt {
    #[structopt(
    long,
    help = "Sets the listening address",
    value_name = "IP:PORT",
    default_value = DEFAULT_LISTENING_ADDRESS,
    parse(try_from_str)
    )]
    addr: SocketAddr,
    #[structopt(
    long,
    help = "Sets the storage engine",
    value_name = "ENGINE-NAME",
    possible_values = & Engine::variants()
    )]
    engine: Option<Engine>,
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Engine {
        kvs,
        sled
    }
}

fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let mut opt = Opt::from_args();
    run(opt)
}

fn run(opt: Opt) -> Result<()> {
    let engine = opt.engine.unwrap_or(DEFAULT_ENGINE);
    info!("kvlocal {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {}", engine);
    info!("Listening on {}", opt.addr);

    match engine {
        Engine::kvs => unimplemented!(),
        Engine::sled => unimplemented!(),
    }
}
