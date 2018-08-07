// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
extern crate clap;
#[macro_use]
extern crate serde_derive;

use clap::{Arg, App, SubCommand};

mod object;

fn main() {
    let matches = App::new("pajero")
                          .version("1.0")
                          .author("GNu. <posgnu@gmail.com>")
                          .about("Awesome packet replayer")
                          .subcommand(SubCommand::with_name("set")
                                      .about("Set the DB")
                                      .version("1.0")
                                      .author("GNu. <posgnu@gmail.com>"))
                          .subcommand(SubCommand::with_name("play")
                                      .about("Replay packet")
                                      .version("1.0")
                                      .author("GNu. <posgnu@gmail.com>")
                                      .arg(Arg::with_name("PCAP")
                                        .help("Path where the pcap file is located")
                                        .required(true)))
                          .get_matches();

    match matches.subcommand() {
        ("set", Some(sub_input)) => {}
        ("play", Some(sub_input)) => {
            let path = sub_input.value_of("PCAP").unwrap();
        }
        _ => {
            println!("Awesome packet replayer, 1.0, GNu. <posgnu@gmail.com>")
        }
    }

    return
}