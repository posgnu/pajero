use pnet::datalink::{pcap, Channel::Ethernet};

use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

use std::fs::{self, read_dir, OpenOptions};
use std::io::prelude::*;
use std::net::IpAddr;
use std::path::Path;

const ROOT_DIR: &str = "./static/packets";

fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn handle_tcp_packet(source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        let des_port = tcp.get_destination();
        let sou_port = tcp.get_source();
        let path = Path::new(ROOT_DIR);
        
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();
        if find_subsequence(tcp.payload(), "ooo".as_bytes()).is_some() {
            println!("Exploited by {:?}", destination);
        }
        writeln!(file, "{:?}", tcp.payload());
    } else {
        println!("[]: Malformed TCP Packet");
    }
}

fn handle_transport_protocol(
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
) {
    match protocol {
        IpNextHeaderProtocols::Tcp => handle_tcp_packet(source, destination, packet),
        _ => {}
    }
}

fn handle_ipv4_packet(ethernet: &EthernetPacket) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.get_next_level_protocol(),
            header.payload(),
        );
    } else {
        println!("[]: Malformed IPv4 Packet");
    }
}

fn handle_ethernet_frame(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(ethernet),
        _ => {}
    }
}

pub fn analyze(tmp_path: String) {
    // Read splitted packet files from the tmp directory
    let paths = fs::read_dir(tmp_path).unwrap();

    // Loop all the splitted packet files
    for path in paths {
        let file_path = path.unwrap().path();
        println!("Name: {}", file_path.display());
        
        
        // Create a channel to receive on
        let (_, mut rx) = match pcap::from_file(file_path, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("packetdump: unhandled channel type: {}"),
            Err(e) => panic!("packetdump: unable to create channel: {}", e),
        };

        loop {
            match rx.next() {
                Ok(packet) => handle_ethernet_frame(&EthernetPacket::new(packet).unwrap()),
                Err(_e) => {
                    println!("Complete to read pcap");
                    break;
                }
            }
        }
    }
}
