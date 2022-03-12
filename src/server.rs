use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use serde_json::Deserializer;
use crate::protocol::Request;
use anyhow::Result;
use crate::protocol;
use crate::storage::StorageEngine;

pub struct Server<T: StorageEngine> {
    engine: T,
}

impl<T: StorageEngine> Server<T> {
    fn new(engine: T) -> Self {
        Server { engine }
    }

    fn run<A: ToSocketAddrs>(&mut self, address: A) -> Result<()> {
        let listener = TcpListener::bind(address)?;
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.serve(stream);
        }
        Ok(())
    }

    fn serve(&mut self, stream: TcpStream) -> Result<()> {
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

        for req in req_reader {
            let req = req?;
            match req {
                Request::Get { key } => match self.engine.get(key) {
                    Ok(x) => {
                        let resp = protocol::GetResponse::Ok(x);
                        serde_json::to_writer(&mut writer, &resp)?
                    }
                    Err(e) => {
                        let resp = protocol::GetResponse::Err(format!("{}", e));
                        serde_json::to_writer(&mut writer, &resp)?
                    }
                }
                Request::Rm { key } => match self.engine.remove(key) {
                    Ok(()) => {
                        let resp = protocol::RmResponse::Ok(());
                        serde_json::to_writer(&mut writer, &resp)?
                    }
                    Err(e) => {
                        let resp = protocol::GetResponse::Err(format!("{}", e));
                        serde_json::to_writer(&mut writer, &resp)?
                    }
                }
                Request::Set { key, value } => match self.engine.set(key, value) {
                    Ok(()) => {
                        let resp = protocol::SetResponse::Ok(());
                        serde_json::to_writer(&mut writer, &resp)?
                    }
                    Err(e) => {
                        let resp = protocol::GetResponse::Err(format!("{}", e));
                        serde_json::to_writer(&mut writer, &resp)?
                    }
                }
            };
            writer.flush()?
        }
        Ok(())
    }
}

