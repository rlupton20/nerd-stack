// ethernet.rs
// Defines structures and functions for working with ethernet headers

pub type MAC = [u8; 6];

fn mac_to_string(mac: &MAC) -> String {
    format!(
        "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
        mac[0],
        mac[1],
        mac[2],
        mac[3],
        mac[4],
        mac[5]
    )
}

#[derive(Debug)]
#[repr(C, packed)]
struct ether_hdr {
    dmac: MAC,
    smac: MAC,
    ethertype: [u8; 2],
}

#[derive(Debug)]
pub struct Ethernet<'a> {
    hdr: &'a ether_hdr,
    contents: *const u8,
}

pub enum PacketType {
    ARP,
    Unknown,
}

impl<'a> Ethernet<'a> {
    const HEADER_LENGTH: usize = 14;

    pub const MTU: usize = 1500;

    pub fn from_buffer(buffer: &[u8; Ethernet::MTU], nbytes: usize) -> Option<Self> {
        if nbytes < Ethernet::HEADER_LENGTH {
            None
        } else {
            let buf_ptr: *const u8 = buffer.as_ptr();
            let ether_hdr_ptr: *const ether_hdr = buf_ptr as *const _;
            let ether_hdr_ref: &ether_hdr = unsafe { &*ether_hdr_ptr };
            let ether: Ethernet = Ethernet {
                hdr: ether_hdr_ref,
                contents: buffer[14..].as_ptr(),
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
}
