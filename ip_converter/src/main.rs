use std::os;
use std::num::{Int, from_str_radix}; //Needed so that pow() works
use std::io::net::ip::IpAddr;

fn ipv4_to_int(ip: [u8; 4]) -> u32 {
    (0..4).fold(0, |sum, x| {sum + (256.pow(ip.len() - x - 1) * (ip[x] as u32))})
}

fn int_to_ipv4(i: u32) -> IpAddr {
    let mut ip = [0u8; 4];
    ip[0] = ((i & 0xff000000) >> 24) as u8;
    ip[1] = ((i & 0x00ff0000) >> 16) as u8;
    ip[2] = ((i & 0x0000ff00) >> 8) as u8;
    ip[3] = (i & 0x000000ff) as u8; 
    IpAddr::Ipv4Addr(ip[0], ip[1], ip[2], ip[3])    
}

fn main() {
    let s = os::args(); 
    if !(s.len() > 1) {
        println!("Usage: ./{} IP_ADDRESS", s[0]);
        return;
    };
    let input: Option<IpAddr> = (s[1].as_slice().trim()).parse();
    match input {
        Some(IpAddr::Ipv4Addr(a, b, c, d)) => {
            let int_ip = ipv4_to_int([a, b, c, d]);
            println!("IP in uint32 format: {} (0x{:08X})", int_ip, int_ip);
            return;       
        },
        Some(IpAddr::Ipv6Addr(..)) => {
            println!("Conversion not supported for IPv6 addresses!");
            return;
        },
        None  => {
            let input: Option<u32> = (s[1].as_slice().trim()).parse();
            match input {
                Some(ip_int) => {
                    println!("IP in dec format: {}", int_to_ipv4(ip_int));
                    return;
                },
                None => {
                    let input =  s[1].as_slice().trim();
                    if input.starts_with("0x") {
                        let input = input.slice_from(2);
                        if let Some(x) = from_str_radix(input, 16) {
                            println!("IP in dec format: {}", int_to_ipv4(x));
                            return;
                        }
                    }
                    println!("Unrecognized IP format!");
                    return;
                }
            }
        }
    }
}
