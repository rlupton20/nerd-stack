extern crate nerd_stack;

use nerd_stack::virt_device::VirtType;
use std::io::{Read, Write};


struct Ethr<A> {
    dmac : [u8 ; 6],
    smac : [u8 ; 6],
    ethertype : u16,
    payload : A
}

trait NetworkPacket : Sized {
    fn new() -> Self;
    fn encode(self) -> Vec<u8>;
    fn decode( bs : Vec<u8> ) -> Result<Self, &'static str>;
}

impl NetworkPacket for Vec<u8> {
    fn new() -> Self {
        Vec::with_capacity(1500)
    }

    fn encode(self) -> Vec<u8> {
        self
    }

    fn decode( mut bs : Vec<u8> ) -> Result<Self, &'static str> {
        Ok(bs)
    }
}

impl<A : NetworkPacket> NetworkPacket for Ethr<A> {
    fn new() -> Self {
        Ethr {
            dmac : [0 ; 6],
            smac : [0 ; 6],
            ethertype : 0,
            payload : A::new()
        }
    }

    fn encode(self) -> Vec<u8> {
        let mut bs : Vec<u8> = Vec::with_capacity(1500);
        bs.extend_from_slice(&self.dmac);
        bs.extend_from_slice(&self.smac);
        bs.append(&mut self.payload.encode());
        // Currently big endian only - add dependent typing
        bs.push(self.ethertype as u8);
        bs.push((self.ethertype >> 8) as u8);
        bs
    }

    fn decode( mut bs : Vec<u8> ) -> Result<Self, &'static str> {
        let mut pkt : Ethr<A> = Self::new();
        if let Some(dmac) = bs.get(0..6) {
            for (i,v) in dmac.iter().enumerate() {
                pkt.dmac[i] = *v;
            }
        }
        if let Some(smac) = bs.get(6..12) {
            for (i,v) in smac.iter().enumerate() {
                pkt.smac[i] = *v;
            }
        }
        else {
            return Err("Bad packet");
        }

        pkt.ethertype = (bs[12] as u16) | (bs[13] as u16) << 8;
        match A::decode(bs.split_off(14)) {
            Ok(p) => {
                pkt.payload = p;
                Ok(pkt)
            }
            Err(_) => Err("Bad payload")
        }
    }
}

fn main() {
    match VirtType::TAP.open("toytap") {
        Ok(mut v) => {
            println!("Device opened");
            let mut buf = [0 ; 4096];
            v.write(&mut buf);
            match v.read(&mut buf) {
                Ok(n) => println!("Read {} bytes", n),
                Err(e) => println!("Error: {}", e)
            }
        }
        Err(e) => println!("Failed to open device: {}", e)
    }
}
