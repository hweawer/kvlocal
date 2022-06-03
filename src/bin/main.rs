use anyhow::{anyhow, Result};
use log::{info, LevelFilter};
use std::env;
use std::fmt::{Display, Formatter, write};
use std::net::SocketAddr;
use std::str::FromStr;
use clap::StructOpt;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

#[derive(clap::Parser, Debug)]
#[clap(name = "kvlocal")]
struct Opt {
    #[clap(
    long,
    help = "Sets the listening address",
    value_name = "IP:PORT",
    default_value = DEFAULT_LISTENING_ADDRESS,
    parse(try_from_str)
    )]
    addr: SocketAddr,
    #[clap(
    long,
    help = "Sets the storage engine",
    value_name = "ENGINE-NAME",
    possible_values = ["kvs", "sled"]
    )]
    engine: Option<Engine>,
}

    #[allow(non_camel_case_types)]
    #[derive(clap::ArgEnum, Debug, Copy, Clone, PartialEq, Eq)]
    enum Engine {
        kvs,
        sled
    }

impl Display for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Engine::kvs => write!(f, "kvs"),
            Engine::sled => write!(f, "sled")
        }
    }
}

impl FromStr for Engine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "kvs" => Ok(Engine::kvs),
            "sled" => Ok(Engine::sled),
            a => Err(anyhow!("Unknown engine type: {}", a))
        }
    }
}

fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let opt = Opt::parse();
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
