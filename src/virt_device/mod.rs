// virt_device :: types and methods for opening and working
// with virtual devices, e.g. TUN and TAP devices

use std::fs::{File, OpenOptions};
use std::os::unix::io::{IntoRawFd, FromRawFd};
use std::io::{Read, Write};
use std::io;
use libc::{c_int, c_char, c_short, ioctl, IF_NAMESIZE, IFF_TUN, IFF_TAP, IFF_NO_PI};

static TUNSETIFF: u64 = 1074025674; // Need to check this

#[repr(C)]
struct IfReq {
    ifr_name: [c_char; IF_NAMESIZE],
    union: [u8; 24],
}

impl IfReq {
    // TODO: This is currently not endian agnostic - fix!
    fn set_ifr_flags(&mut self, flags: c_short) {
        // Zero the flags and copy the two bytes of flags
        self.union = [0; 24];
        self.union[0] = flags as u8;
        self.union[1] = (flags >> 8) as u8;
    }

    fn new() -> Self {
        let name: [c_char; IF_NAMESIZE] = [0 as c_char; IF_NAMESIZE];
        IfReq {
            ifr_name: name,
            union: [0; 24],
        }
    }

    fn from_name(name: &str) -> Result<IfReq, &'static str> {
        if name.len() >= IF_NAMESIZE {
            Err("Device name too long")
        } else {
            let mut ifreq: IfReq = IfReq::new();
            for (i, c) in name.as_bytes().iter().enumerate() {
                ifreq.ifr_name[i] = *c as c_char;
            }
            Ok(ifreq)
        }
    }
}

pub enum VirtType {
    TUN,
    TAP,
}

pub struct Virt(VirtDevice);

enum VirtDevice {
    TUN { f: File },
    TAP { f: File },
}

impl Read for Virt {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let Virt(ref mut device) = *self;
        match *device {
            VirtDevice::TUN { ref mut f } => f.read(buf),
            VirtDevice::TAP { ref mut f } => f.read(buf),
        }
    }
}

impl Write for Virt {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let Virt(ref mut device) = *self;
        match *device {
            VirtDevice::TUN { ref mut f } => f.write(buf),
            VirtDevice::TAP { ref mut f } => f.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let Virt(ref mut device) = *self;
        match *device {
            VirtDevice::TUN { ref mut f } => f.flush(),
            VirtDevice::TAP { ref mut f } => f.flush(),
        }
    }
}


impl VirtType {
    fn flags(&self) -> c_short {
        match *self {
            VirtType::TUN => IFF_TUN | IFF_NO_PI,
            VirtType::TAP => IFF_TAP | IFF_NO_PI,
        }
    }

    fn perform_ioctl(fd: c_int, ifreq: IfReq) -> Result<c_int, &'static str> {
        unsafe {
            let rc: i32 = ioctl(fd, TUNSETIFF, &ifreq);
            if rc < 0 {
                Err("Failed on ioctl")
            } else {
                Ok(fd)
            }
        }
    }

    fn wrap(&self, f: File) -> Virt {
        match *self {
            VirtType::TUN => Virt(VirtDevice::TUN { f: f }),
            VirtType::TAP => Virt(VirtDevice::TAP { f: f }),
        }
    }

    fn lift(fd: c_int) -> File {
        unsafe { File::from_raw_fd(fd) }
    }


    pub fn open(&self, path: &str) -> Result<Virt, &'static str> {
        let flags: c_short = self.flags();

        IfReq::from_name(path).and_then(|mut ifreq: IfReq| {
            ifreq.set_ifr_flags(flags);

            match OpenOptions::new().read(true).write(true).open(
                "/dev/net/tun",
            ) {

                Ok(f) => {
                    let fd: c_int = f.into_raw_fd();
                    VirtType::perform_ioctl(fd, ifreq).map(VirtType::lift).map(
                        |f: File| self.wrap(f),
                    )
                }

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
    assert!((1 & VirtType::TUN.flags()) == 1);
    assert!((1 & VirtType::TAP.flags()) == 0);
}

#[test]
// TODO: This assumes little endianess
fn test_flag_setting_in_ifreq() {
    let mut ifreq: IfReq = IfReq::new();
    ifreq.set_ifr_flags(IFF_TAP | IFF_NO_PI);
    assert!(ifreq.union[1] == (IFF_NO_PI >> 8) as u8);
    assert!(ifreq.union[0] == IFF_TAP as u8);

}
