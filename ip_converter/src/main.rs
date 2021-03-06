use std::net::{Ipv4Addr, Ipv6Addr};

fn ipv4_to_int(ip: Ipv4Addr) -> u32 {
    let ip_oct = ip.octets();
    (0..4).fold(0, |sum, x| {sum + (256u32.pow((ip_oct.len() - x - 1) as u32) * (ip_oct[x] as u32))})
}

fn int_to_ipv4(i: u32) -> Ipv4Addr {
    let mut ip = [0u8; 4];
    ip[0] = ((i & 0xff000000) >> 24) as u8;
    ip[1] = ((i & 0x00ff0000) >> 16) as u8;
    ip[2] = ((i & 0x0000ff00) >> 8) as u8;
    ip[3] = (i & 0x000000ff) as u8; 
    Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3])
}

struct Output {
    ipv4: Ipv4Addr,
    int32: u32,
    }

fn main() {
    let mut output: Output = Output{ipv4: Ipv4Addr::new(0, 0, 0, 0), int32: 0};
    // read input args and take the first 2 (prog name and argument)
    // and transform it to vector
    let s: Vec<_> = std::env::args().take(2).collect(); 
    if s.len() == 2 {
        let input = s[1].clone();
        if let Ok(ip) = input.trim().parse::<Ipv4Addr>() {
            output.ipv4 = ip;
            output.int32 = ipv4_to_int(ip);       
        } else if let Ok(_) = input.trim().parse::<Ipv6Addr>() {
            println!("Conversion not supported for IPv6 addresses!");
            return;
        } else {
            match input.trim().parse() {
                Ok(ip_int) => {
                    output.ipv4 = int_to_ipv4(ip_int);
                    output.int32 = ip_int;
                },
                Err(_) => {
                    if input.trim().starts_with("0x") {
                        if let Ok(x) = u32::from_str_radix(&input[2..], 16) {
                            output.ipv4 = int_to_ipv4(x);
                            output.int32 = x;
                        } else {
                            println!("Invalid value: 0x{}", input);
                            return;
                        }
                    } else {
                        println!("Unrecognized IP format!");
                        return;
                    }
                }
            }
        }
        println!("{}, 0x{:08X}, {}", output.ipv4, output.int32, output.int32);
    } else {
        println!("Usage: {} IP_ADDRESS", s[0]);
        return;
   }     
}


#[test]
fn test_from_ipv4() {
    assert!(3232236035u32 == ipv4_to_int(Ipv4Addr::new(192, 168, 2, 3)));
}

#[test]
fn test_from_int() {
    assert!(int_to_ipv4(3232236035u32) == Ipv4Addr::new(192, 168,2, 3));
}


