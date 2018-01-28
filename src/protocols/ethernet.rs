// ethernet.rs
// Defines structures and functions for working with ethernet headers

use address::{MAC, mac_to_string};

#[repr(C, packed)]
#[derive(Debug)]
struct ether_hdr {
    dmac: MAC,
    smac: MAC,
    ethertype: [u8; 2],
}

#[derive(Debug)]
pub struct Ethernet<'a> {
    hdr: &'a ether_hdr,
    contents: &'a [u8],
}

pub enum PacketType {
    ARP,
    Unknown,
}

impl<'a> Ethernet<'a> {
    pub const HEADER_LENGTH: usize = 14;

    pub const MTU: usize = 1500;

    pub fn from_buffer(buffer: &'a [u8]) -> Option<Self> {
        if buffer.len() < Ethernet::HEADER_LENGTH {
            None
        } else {
            let buf_ptr: *const u8 = buffer.as_ptr();
            let ether_hdr_ptr: *const ether_hdr = buf_ptr as *const _;
            let ether_hdr_ref: &ether_hdr = unsafe { &*ether_hdr_ptr };
            let ether: Ethernet<'a> = Ethernet {
                hdr: ether_hdr_ref,
                contents: &buffer[14..],
            };
            Some(ether)
        }
    }

    fn ethertype(&self) -> u16 {
        self.hdr.ethertype[1] as u16 | (self.hdr.ethertype[0] as u16) << 8
    }

    pub fn payload_type(&self) -> PacketType {
        match self.ethertype() {
            0x0806 => PacketType::ARP,
            _ => PacketType::Unknown,
        }
    }

    pub fn source_mac(&self) -> String {
        mac_to_string(&self.hdr.smac)
    }

    pub fn dest_mac(&self) -> String {
        mac_to_string(&self.hdr.dmac)
    }

    pub fn contents(&self) -> &[u8] {
        self.contents
    }
}
