use pnet::datalink::{pcap, Channel::Ethernet};

use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

use std::fs::{self, create_dir_all, read_dir, OpenOptions};
use std::io::prelude::*;
use std::net::IpAddr;
use std::path::Path;
use std::str;

use crate::conf::Config;

const ROOT_DIR: &str = "./static/packets";

fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn handle_tcp_packet(
    source: IpAddr,
    destination: IpAddr,
    packet: &[u8],
    file_name: String,
    round: u8,
) {
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        let des_port = tcp.get_destination();
        let sou_port = tcp.get_source();

        let source_name = match Config::team_ip_to_name(source.to_string()) {
            Ok(name) => name,
            Err(_) => "local".to_string(),
        };

        let destination_name = match Config::team_ip_to_name(destination.to_string()) {
            Ok(name) => name,
            Err(_) => "local".to_string(),
        };

        let service_name = match Config::service_port_to_name(des_port.into()) {
            Ok(port) => port,
            Err(_) => match Config::service_port_to_name(sou_port.into()) {
                Ok(port) => port,
                Err(_) => "unknown".to_string(),
            },
        };

        let dir_team_name = if source_name != "local" {
            source_name.clone()
        } else {
            if destination_name == "local" {
                "unknow".to_string()
            } else {
                destination_name.clone()
            }
        };

        let path = Path::new(ROOT_DIR)
            .join(dir_team_name.clone())
            .join(service_name.clone())
            .join("Round ".to_owned() + &round.to_string())
            .join(file_name);

        create_dir_all(
            Path::new(ROOT_DIR)
                .join(dir_team_name)
                .join(service_name.clone())
                .join("Round ".to_owned() + &round.to_string()),
        );

        let flag = match Config::service_name_to_flag(service_name) {
            Ok(flag) => flag,
            // Default flag
            Err(_) => "DEFCON{".to_string(),
        };

        let mut file;
        // Contain flag
        if find_subsequence(tcp.payload(), flag.as_bytes()).is_some() {
            file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap();
        } else {
            // Open file "team/service/round/[packets]""
            file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap();
        }

        match str::from_utf8(tcp.payload()) {
            Ok(pay) => {
                writeln!(file, "{} -> {}", source_name, destination_name);
                writeln!(file, "{:?}", pay);
                writeln!(file, "");
            }
            Err(err) => {
                writeln!(file, "{} -> {}", source_name, destination_name);
                writeln!(file, "{:?}", tcp.payload());
                writeln!(file, "");
            }
        };
    } else {
        println!("[]: Malformed TCP Packet");
    }
}

fn handle_transport_protocol(
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
    file_name: String,
    round: u8,
) {
    match protocol {
        IpNextHeaderProtocols::Tcp => {
            handle_tcp_packet(source, destination, packet, file_name, round)
        }
        _ => {}
    }
}

fn handle_ipv4_packet(ethernet: &EthernetPacket, file_name: String, round: u8) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.get_next_level_protocol(),
            header.payload(),
            file_name,
            round,
        );
    } else {
        println!("[]: Malformed IPv4 Packet");
    }
}

fn handle_ethernet_frame(ethernet: &EthernetPacket, file_name: String, round: u8) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(ethernet, file_name, round),
        _ => {}
    }
}

pub fn analyze(tmp_path: String, round: u8) {
    // Read splitted packet files from the tmp directory
    let paths = fs::read_dir(tmp_path).unwrap();

    // Loop all the splitted packet files
    for path in paths {
        let file_path = path.unwrap().path();
        let file_name = file_path.file_stem().unwrap().to_str().unwrap().to_owned() + ".txt";

        // Create a channel to receive on
        let (_, mut rx) = match pcap::from_file(file_path.clone(), Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("packetdump: unhandled channel type: {}"),
            Err(e) => panic!("packetdump: unable to create channel: {}", e),
        };

        loop {
            match rx.next() {
                Ok(packet) => handle_ethernet_frame(
                    &EthernetPacket::new(packet).unwrap(),
                    file_name.to_string(),
                    round,
                ),
                Err(_e) => {
                    println!("Complete to read pcap");
                    break;
                }
            }
        }
    }
}
