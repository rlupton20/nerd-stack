use address::{MAC, IPv4, mac_to_string, ipv4_to_string};

use std::slice;

#[repr(C, packed)]
struct arp_hdr {
    hwtype: u16,
    protype: u16,
    hwsize: u8,
    prosize: u8,
    opcode: u16,
}

pub struct ARP<'a> {
    header: &'a arp_hdr,
    contents: &'a [u8],
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HWType {
    Ethernet,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProType {
    IPv4,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Opcode {
    Request,
    Reply,
}

impl Opcode {
    fn encode(&self) -> u16 {
        match self {
            Request => 0x0100,
            Reply => 0x0200,
        }
    }
}

impl<'a> ARP<'a> {
    const HEADER_LENGTH: usize = 8;

    pub fn from_buffer(buffer: &'a [u8]) -> Option<Self> {
        if buffer.len() < ARP::HEADER_LENGTH {
            None
        } else {
            let buf_ptr: *const u8 = buffer.as_ptr();
            let arp_hdr_ptr: *const arp_hdr = buf_ptr as *const _;
            let arp_hdr_ref: &arp_hdr = unsafe { &*arp_hdr_ptr };
            let arp: ARP<'a> = ARP {
                header: arp_hdr_ref,
                contents: &buffer[ARP::HEADER_LENGTH..],
            };
            Some(arp)
        }
    }

    // TODO: Make agnostic to endianess
    pub fn hwtype(&self) -> Option<HWType> {
        match self.header.hwtype {
            0x0100 => Some(HWType::Ethernet),
            _ => None,
        }
    }

    // TODO: Make agnostic to endianess
    pub fn protype(&self) -> Option<ProType> {
        match self.header.protype {
            0x0008 => Some(ProType::IPv4),
            _ => None,
        }
    }

    // TODOL Make agnostic to endianess
    pub fn opcode(&self) -> Option<Opcode> {
        match self.header.opcode {
            0x0100 => Some(Opcode::Request),
            _ => None,
        }
    }

    pub fn contents(&self) -> &[u8] {
        self.contents
    }
}

#[repr(C, packed)]
struct arp_ipv4_parts {
    smac: MAC,
    sip: IPv4,
    dmac: MAC,
    dip: IPv4,
}

pub struct ArpIPv4<'a> {
    parts: &'a arp_ipv4_parts,
}

impl<'a> ArpIPv4<'a> {
    const LENGTH: usize = 20;

    pub fn from_buffer(buffer: &'a [u8]) -> Option<Self> {
        if buffer.len() != ArpIPv4::LENGTH {
            None
        } else {
            let buf_ptr: *const u8 = buffer.as_ptr();
            let arp_ipv4_ptr: *const arp_ipv4_parts = buf_ptr as *const _;
            let arp_ipv4_ref: &arp_ipv4_parts = unsafe { &*arp_ipv4_ptr };
            let arp_ipv4: ArpIPv4 = ArpIPv4 { parts: arp_ipv4_ref };
            Some(arp_ipv4)
        }
    }

    pub fn source_mac(&self) -> &MAC {
        &self.parts.smac
    }

    pub fn destination_mac(&self) -> &MAC {
        &self.parts.dmac
    }

    pub fn source_ip(&self) -> &IPv4 {
        &self.parts.sip
    }

    pub fn destination_ip(&self) -> &IPv4 {
        &self.parts.dip
    }
}
