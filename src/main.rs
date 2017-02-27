extern crate libc;

use std::fs::OpenOptions;
use std::os::unix::io::IntoRawFd;
use libc::{c_char, c_short, c_int, ioctl, IF_NAMESIZE};

static IFFTAP : c_short = 2;
static IFF_NO_PI : c_short = 4096;
static TUNSETIFF : u64 = 1074025674; // Need to check this

#[repr(C)]
struct IfReq {
    ifr_name : [c_char; IF_NAMESIZE],
    union : [u8; 24]
}

impl IfReq {
    fn ifr_flags(mut self, flags : c_short) {
        // Zero the flags and copy the two bytes of flags
        self.union = [0; 24];
        self.union[0] = flags as u8;
        self.union[1] = (flags << 8) as u8;
    }

    fn new() -> Self {
        let name : [c_char ; IF_NAMESIZE] = [0 as c_char ; IF_NAMESIZE];
        IfReq {
            ifr_name : name,
            union : [0 ; 24]
        }
    }

    fn from_name(name : &str) -> Option<IfReq> {
        if name.len() >= IF_NAMESIZE {
            None
        }
        else {
            let mut ifreq : IfReq = IfReq::new();
            for (i,c) in name.as_bytes().iter().enumerate() {
                ifreq.ifr_name[i] = *c as c_char;
            }
            Some(ifreq)
        }
    }
}



fn main() {
    let mut ifreq : IfReq;

    let ifreq : Option<IfReq> = IfReq::from_name("toytap");

    match OpenOptions::new().write(true).open("/dev/net/tun") {
        Ok(t) => {
            println!("OK");
            unsafe {
                ioctl(t.into_raw_fd(), TUNSETIFF, ifreq);
            }
        }
        Err(e) => println!("Failed to open tun device: {}", e)
    }


}
