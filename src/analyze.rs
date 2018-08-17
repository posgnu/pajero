use pnet::datalink::{pcap, Channel::Ethernet};

use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::net::IpAddr;
use std::path::Path;

fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where for<'a> &'a [T]: PartialEq 
{
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn handle_tcp_packet(source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        let port = tcp.get_destination();
        let path = Path::new("./packet")
            .join(port.to_string())
            .join(source.to_string());
        fs::create_dir_all(path.parent().unwrap());
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

pub fn analyze(path: String) {
    let path = Path::new(&path);
    // Create a channel to receive on
    let (_, mut rx) = match pcap::from_file(path, Default::default()) {
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
