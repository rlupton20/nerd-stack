// virt_device :: types and methods for opening and working
// with virtual devices, e.g. TUN and TAP devices

use std::fs::{File, OpenOptions};
use std::os::unix::io::{IntoRawFd, FromRawFd};
use libc::{c_int, c_char, c_short, ioctl, IF_NAMESIZE};

static IFFTUN : c_short = 1;
static IFFTAP : c_short = 2;
static IFF_NO_PI : c_short = 4096;
static TUNSETIFF : u64 = 1074025674; // Need to check this

#[repr(C)]
struct IfReq {
    ifr_name : [c_char; IF_NAMESIZE],
    union : [u8; 24]
}

impl IfReq {
    fn set_ifr_flags(&mut self, flags : c_short) {
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

    fn from_name(name : &str) -> Result<IfReq, &'static str> {
        if name.len() >= IF_NAMESIZE {
            Err("Device name too long")
        }
        else {
            let mut ifreq : IfReq = IfReq::new();
            for (i,c) in name.as_bytes().iter().enumerate() {
                ifreq.ifr_name[i] = *c as c_char;
            }
            Ok(ifreq)
        }
    }
}

pub enum VirtType {
    TUN,
    TAP
}

type FileDescriptor = c_int;

pub struct Virt(VirtDevice);

enum VirtDevice {
    TUN { fd : FileDescriptor },
    TAP { fd : FileDescriptor }
}

impl VirtType {
    fn flags(&self) -> c_short {
        match *self {
            VirtType::TUN => IFFTUN | IFF_NO_PI,
            VirtType::TAP => IFFTAP | IFF_NO_PI
        }
    }

    unsafe fn perform_ioctl(&self, fd : c_int, ifreq : IfReq)
                       -> Result<Virt, &'static str> {
        let rc : i32 = ioctl(fd, TUNSETIFF, ifreq);
        if rc < 0 {
            Err("Failed on ioctl")
        }
        else {
            match *self {
                VirtType::TUN => Ok(Virt(VirtDevice::TUN { fd : fd, })),
                VirtType::TAP => Ok(Virt(VirtDevice::TAP { fd : fd, }))
            }
        }
    }
        

    pub fn open(&self, path : &str) -> Result<Virt, &'static str> {
        let flags : c_short = self.flags();

        IfReq::from_name(path)
            .and_then(| mut ifreq : IfReq | {
                ifreq.set_ifr_flags(flags);

                match OpenOptions::new().write(true).open("/dev/net/tun") {

                    Ok(f) => unsafe {
                        let fd : c_int = f.into_raw_fd();
                        self.perform_ioctl(fd, ifreq)
                    },

                    Err(e) => {
                        println!("Can't open device: {}", e);
                        Err("Failed to open device")
                    }
                }
            })
    }
}

#[test]
fn test_virt_type_flags() {
    assert!( (1 & VirtType::TUN.flags()) == 1 );
    assert!( (1 & VirtType::TAP.flags()) == 0 );
}
