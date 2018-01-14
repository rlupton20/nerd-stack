extern crate nerd_stack;

use nerd_stack::virt_device::VirtType;
use nerd_stack::protocols::ethernet::{Ethernet, PacketType};
use nerd_stack::protocols::arp::{ARP, ARP_IPv4, HWType, ProType};

use std::io::Read;


fn ethernet_dispatch(buffer: &[u8]) -> () {
    let nbytes: usize = buffer.len();
    match Ethernet::from_buffer(buffer) {
        Some(pkt) => {
            match pkt.payload_type() {
                PacketType::ARP => {
                    println!(
                        "--> ARP :: {} bytes :: src {} dest {}",
                        nbytes,
                        pkt.source_mac(),
                        pkt.dest_mac()
                    );
                    arp_handler(pkt.contents(), HWType::Ethernet);
                }
                PacketType::Unknown => {
                    println!(
                        "--> UNKNOWN :: {} bytes :: src {} dest {}",
                        nbytes,
                        pkt.source_mac(),
                        pkt.dest_mac()
                    );
                }
            }
        }
        None => {
            println!("Ethernet packet malformed :: ignoring");
        }
    }
}


fn arp_handler(buffer: &[u8], hwtype: HWType) -> () {
    match ARP::from_buffer(buffer) {
        Some(pkt) => {
            if let Some(hw) = pkt.hwtype() {
                if hw == hwtype {
                    match pkt.protype() {
                        Some(ProType::IPv4) => arp_ipv4_handler(pkt.contents()),
                        _ => println!("Ignoring"),
                    }
                } else {
                    println!("Bad ARP packet");
                }
            } else {
                println!("Bad ARP packet");
            }
        }
        None => {
            println!("Invalid ARP packet");
        }
    }
}


fn arp_ipv4_handler(buffer: &[u8]) -> () {
    match ARP_IPv4::from_buffer(buffer) {
        Some(pkt) => {
            println!(
                "           Who has {}? Tell {}",
                pkt.destination_ip(),
                pkt.source_ip()
            );
        }
        None => println!("Bad ARP for IPv4 address"),
    }
}


fn main() {
    match VirtType::TAP.open("toytap") {
        Ok(mut v) => {
            println!("Device opened");
            loop {
                let mut buf: [u8; Ethernet::MTU] = [0; Ethernet::MTU];
                match v.read(&mut buf) {
                    Ok(n) => {
                        let packet: &[u8] = &buf[..n];
                        ethernet_dispatch(packet);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        Err(e) => println!("Failed to open device: {}", e),
    }
}
