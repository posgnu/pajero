extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate pnet;
extern crate rocksdb;

use analyze::analyze;
use clap::{App, Arg, SubCommand};
use play::play;
use set::set_team_info;

mod analyze;
mod object;
mod play;
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
                .author("GNu. <posgnu@gmail.com>"),
        )
        .subcommand(
            SubCommand::with_name("analyze")
                .about("Analyze packet")
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
        ("set", Some(_sub_input)) => match set_team_info() {
            Ok(()) => println!("Success setting!"),
            Err(s) => println!("Something happen wrong!: {}", s),
        },
        ("play", Some(_sub_input)) => match play() {
            Ok(()) => println!("Success playing!"),
            Err(_) => println!("Fail playing!"),
        },
        ("analyze", Some(sub_input)) => {
            let path: String = sub_input.value_of("PCAP").unwrap().to_string();

            analyze(path);
        }
        _ => println!("Awesome packet replayer, 1.0, GNu. <posgnu@gmail.com>"),
    }

    return;
}
