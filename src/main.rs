extern crate nerd_stack;

use nerd_stack::virt_device::VirtType;
use nerd_stack::protocols::ethernet::{Ethernet, PacketType};
use nerd_stack::protocols::arp::{ARP, ArpIPv4, HWType, ProType, Opcode};

use nerd_stack::address::{MAC, IPv4, mac_to_string, ipv4_to_string};

use std::collections::BTreeMap;

use std::io::Read;

type ArpTable = BTreeMap<(ProType, [u8; 4]), MAC>;


fn ethernet_dispatch(arp_table: &mut ArpTable, buffer: &[u8]) -> () {
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
                    arp_handler(arp_table, pkt.contents(), HWType::Ethernet);
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

fn arp_handler(arp_table: &mut ArpTable, buffer: &[u8], hwtype: HWType) -> () {
    match ARP::from_buffer(buffer) {
        Some(pkt) => {
            if let (Some(hw), Some(opcode)) = (pkt.hwtype(), pkt.opcode()) {
                if hw == hwtype {
                    // TODO: Check hwsize
                    match pkt.protype() {
                        Some(ProType::IPv4) => arp_ipv4_handler(arp_table, pkt.contents(), opcode),
                        _ => println!("Ignoring"),
                    }
                } else {
                    println!("Unhandled ARP packet");
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


fn arp_ipv4_handler(arp_table: &mut ArpTable, buffer: &[u8], opcode: Opcode) -> () {
    match ArpIPv4::from_buffer(buffer) {
        Some(pkt) => {

            // For debugging
            if opcode == Opcode::Request {
                println!(
                    "           Who has {}? Tell {}",
                    ipv4_to_string(pkt.destination_ip()),
                    ipv4_to_string(pkt.source_ip())
                );
            }

            let mut merge_flag: bool = false;
            if arp_table.contains_key(&(ProType::IPv4, *pkt.source_ip())) {
                println!("           Updating MAC for sender");
                arp_table.insert((ProType::IPv4, *pkt.source_ip()), *pkt.source_mac());
                merge_flag = true;
            }

            if *pkt.destination_ip() == ([10, 0, 0, 1] as [u8; 4]) {
                println!("           ARP destined for me");
                if merge_flag == false {
                    arp_table.insert((ProType::IPv4, *pkt.source_ip()), *pkt.source_mac());
                }

                if opcode == Opcode::Request {
                    println!("           Need to reply");
                }
            }

        }
        _ => println!("Ignoring"),
    }
}


fn main() {
    let mut arp_table: ArpTable = BTreeMap::new();

    match VirtType::TAP.open("toytap") {
        Ok(mut v) => {
            println!("Device opened");
            loop {
                let mut buf: [u8; Ethernet::MTU] = [0; Ethernet::MTU];
                match v.read(&mut buf) {
                    Ok(n) => {
                        let packet: &[u8] = &buf[..n];
                        ethernet_dispatch(&mut arp_table, packet);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        Err(e) => println!("Failed to open device: {}", e),
    }
}
