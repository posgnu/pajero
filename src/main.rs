extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate pnet;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use conf::Config;
use splitter::split_pcap;

mod analyze;
mod conf;
mod splitter;

fn main() {
    let matches = App::new("pajero")
        .version("1.0")
        .author("GNu. <posgnu@gmail.com>")
        .about("Awesome packet replayer")
        .subcommand(SubCommand::with_name("replay").about("Replay packet"))
        .subcommand(
            SubCommand::with_name("analyze")
                .about("Analyze packet")
                .arg(
                    Arg::with_name("*.pcap")
                        .help("Path where the pcap file is located")
                        .required(true),
                )
                .arg(
                    Arg::with_name("round")
                        .help("Round of packets")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("replay", Some(_sub_input)) => {
            println!("Not Implemented");
        }
        ("analyze", Some(sub_input)) => {
            let path: String = sub_input.value_of("*.pcap").unwrap().to_string();
            let round: u8 = sub_input.value_of("round").unwrap().parse::<u8>().unwrap();

            split_pcap(path, round);
        }
        _ => println!("Awesome packet replayer, 1.0, GNu. <posgnu@gmail.com>"),
    }
    return;
}
