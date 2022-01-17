mod storage;
mod log;

use std::io;
use storage::KVStorage;

const SET: &str = "set";
const RM: &str = "rm";
const GET: &str = "get";
const EXIT: &str = "exit";

pub fn run() {
    let mut input = String::new();
    let mut storage: KVStorage<String> = KVStorage::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read command");
        input = input.trim().to_string();
        let args: Vec<&str> = input.trim().split(' ').collect();
        match args[0] {
            SET => storage.insert(args[1].to_string(), args[2].to_string()),
            RM => {
                if let Err(x) = storage.delete(args[1]) {
                    println!("{}", x)
                }
            }
            GET => match storage.get(args[1]) {
                Some(x) => println!("{}", x),
                None => eprintln!("Key wasn't found"),
            },
            EXIT => return,
            x => eprintln!("Unknown command {}", x),
        }
        input.clear()
    }
}
