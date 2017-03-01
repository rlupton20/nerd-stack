extern crate nerd_stack;

use nerd_stack::virt_device::VirtType;

fn main() {
    match VirtType::TAP.open("toytap") {
        Ok(_) => println!("Device opened"),
        Err(e) => println!("Failed to open device: {}", e)
    }
}
