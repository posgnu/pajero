use bincode::deserialize;
use crate::object::*;
use rocksdb::{IteratorMode, DB};

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn play() -> Result<()> {
    let db = DB::open_default("./db").unwrap();
    let iter = db.iterator(IteratorMode::Start);

    for (_key, value) in iter {
        let provider: ServiceProvider = deserialize(&value).unwrap();
        println!("{} : {}", provider.team.name, provider.connection);

        for service in provider.service_variants {
            let mut connect = provider.connection.clone();
            connect.push_str(&":".to_string());
            connect.push_str(&service.port);
            println!("{}", connect);

            let iter1 = db.iterator(IteratorMode::Start);

            for (_key1, value1) in iter1 {
                let provider1: ServiceProvider = deserialize(&value1).unwrap();
                let path = Path::new("./packet/")
                    .join(service.port.clone())
                    .join(provider1.connection);

                match File::open(path) {
                    Ok(file) => {
                        for line in BufReader::new(file).lines() {
                            match line {
                                // To be Implemented
                                Ok(l) => println!("{:?}", l),
                                Err(_) => break,
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }

    Ok(())
}
