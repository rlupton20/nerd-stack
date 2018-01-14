extern crate nerd_stack;

use nerd_stack::virt_device::VirtType;
use nerd_stack::protocols::ethernet::{Ethernet, PacketType};

use std::io::Read;


fn ethernet_dispatch(buffer: [u8; 4096], nbytes: usize) -> () {
    match Ethernet::from_buffer(buffer, nbytes) {
        Some(pkt) => {
            match pkt.payload_type() {
                PacketType::ARP => {
                    println!(
                        "--> ARP :: {} bytes :: src {} dest {}",
                        nbytes,
                        pkt.source_mac(),
                        pkt.dest_mac()
                    );
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

fn main() {
    match VirtType::TAP.open("toytap") {
        Ok(mut v) => {
            println!("Device opened");
            loop {
                let mut buf: [u8; 4096] = [0; 4096];
                match v.read(&mut buf) {
                    Ok(n) => {
                        ethernet_dispatch(buf, n);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        Err(e) => println!("Failed to open device: {}", e),
    }
}
