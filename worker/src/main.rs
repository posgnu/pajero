#![feature(proc_macro_hygiene, decl_macro)]

extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate pnet;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate rocket;

use analyze::analyze;
use clap::{App, Arg, SubCommand};
use conf::Config;
use play::play;
use serve::serve;

mod analyze;
mod conf;
mod object;
mod play;
mod serve;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let matches = App::new("pajero")
        .version("1.0")
        .author("GNu. <posgnu@gmail.com>")
        .about("Awesome packet replayer")
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
                    Arg::with_name("*.pcap")
                        .help("Path where the pcap file is located")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .about("Run API provider")
                .version("1.0")
                .author("GNu. <posgnu@gmail.com>"),
        )
        .get_matches();

    match matches.subcommand() {
        ("play", Some(_sub_input)) => match play() {
            Ok(()) => println!("Success playing!"),
            Err(_) => println!("Fail playing!"),
        },
        ("analyze", Some(sub_input)) => {
            let path: String = sub_input.value_of("*.pcap").unwrap().to_string();

            analyze(path);
        }
        ("serve", Some(_sub_input)) => serve(),
        _ => println!("Awesome packet replayer, 1.0, GNu. <posgnu@gmail.com>"),
    }
    return;
}
