#[derive(Debug)]
#[repr(C, packed)]
struct ether_hdr {
    dmac: [u8; 6],
    smac: [u8; 6],
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
    pub fn from_buffer(buffer: [u8; 4096], nbytes: usize) -> Option<Self> {
        if nbytes < 14 {
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
    pub fn dest_mac(&self) -> String {
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
