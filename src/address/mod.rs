pub type MAC = [u8; 6];
pub type IPv4 = [u8; 4];

pub fn mac_to_string(mac: &MAC) -> String {
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

pub fn ipv4_to_string(ip: &IPv4) -> String {
    format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
}
