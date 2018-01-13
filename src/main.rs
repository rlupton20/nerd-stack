extern crate nerd_stack;

use nerd_stack::virt_device::VirtType;
use std::io::{Read, Write};

#[derive(Debug)]
#[repr(C, packed)]
struct ether_hdr {
    dmac: [u8; 6],
    smac: [u8; 6],
    ethertype: [u8; 2],
}

#[derive(Debug)]
struct Ethernet<'a> {
    hdr: &'a ether_hdr,
    contents: *const u8,
}

enum L3PacketType {
    ARP,
    Unknown,
}

impl<'a> Ethernet<'a> {
    fn ethertype(&self) -> u16 {
        self.hdr.ethertype[1] as u16 | (self.hdr.ethertype[0] as u16) << 8
    }
    fn payload_type(&self) -> L3PacketType {
        match self.ethertype() {
            0x0806 => L3PacketType::ARP,
            _ => L3PacketType::Unknown,
        }
    }
    fn source_mac(&self) -> String {
        format!(
            "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
            self.hdr.smac[0],
            self.hdr.smac[1],
            self.hdr.smac[2],
            self.hdr.smac[3],
            self.hdr.smac[4],
            self.hdr.smac[5]
        )
    }
    fn dest_mac(&self) -> String {
        format!(
            "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
            self.hdr.dmac[0],
            self.hdr.dmac[1],
            self.hdr.dmac[2],
            self.hdr.dmac[3],
            self.hdr.dmac[4],
            self.hdr.dmac[5]
        )
    }
}

fn dispatch(pkt: Ethernet, nbytes: usize) -> () {
    match pkt.payload_type() {
        L3PacketType::ARP => {
            println!(
                "--> ARP :: {} bytes :: src {} dest {}",
                nbytes,
                pkt.source_mac(),
                pkt.dest_mac()
            );
        }
        L3PacketType::Unknown => {
            println!(
                "--> UNKNOWN :: {} bytes :: src {} dest {}",
                nbytes,
                pkt.source_mac(),
                pkt.dest_mac()
            );
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
                        let buf_ptr: *const u8 = buf.as_ptr();
                        let ether_hdr_ptr: *const ether_hdr = buf_ptr as *const _;
                        let ether_hdr_ref: &ether_hdr = unsafe { &*ether_hdr_ptr };
                        let ether: Ethernet = Ethernet {
                            hdr: ether_hdr_ref,
                            contents: buf[14..].as_ptr(),
                        };
                        dispatch(ether, n);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        Err(e) => println!("Failed to open device: {}", e),
    }
}
