extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate rocksdb;

use clap::{App, Arg, SubCommand};
use set::SetTeamInfo;

mod object;
mod set;

fn main() {
    let matches = App::new("pajero")
        .version("1.0")
        .author("GNu. <posgnu@gmail.com>")
        .about("Awesome packet replayer")
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the DB")
                .version("1.0")
                .author("GNu. <posgnu@gmail.com>"),
        )
        .subcommand(
            SubCommand::with_name("play")
                .about("Replay packet")
                .version("1.0")
                .author("GNu. <posgnu@gmail.com>")
                .arg(
                    Arg::with_name("PCAP")
                        .help("Path where the pcap file is located")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(sub_input)) => {
            SetTeamInfo()?;
        }
        ("play", Some(sub_input)) => {
            let path = sub_input.value_of("PCAP").unwrap();
            println!("{}", path);
        }
        _ => println!("Awesome packet replayer, 1.0, GNu. <posgnu@gmail.com>"),
    }

    return;
}
