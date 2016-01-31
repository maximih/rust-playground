extern crate octavo;

use octavo::digest::Digest;
use octavo::digest::md5::Md5;
use std::fs::File;
use std::io::{BufReader, Read};

const BUFFER_SIZE: usize = 4096;

fn main() {
    let s: Vec<_> = std::env::args().take(2).collect(); 
    if s.len() == 2 {

        let mut result = vec![0; Md5::output_bytes()];
        let mut md5 = Md5::default();

        let file = File::open(&s[1]).unwrap();
        let file_len = file.metadata().map(|m| m.len()).unwrap();

        let mut reader = BufReader::new(file);
        let mut buf = vec![0; BUFFER_SIZE];

        let mut bytes_read: u64 = 0;

        while bytes_read < file_len {
            let curr_read = reader.read(&mut buf).unwrap() as u64;
            md5.update(&buf[0..curr_read as usize]);
            bytes_read = bytes_read + curr_read;
        }

        md5.result(&mut result);
        for byte in result {
            print!("{:x}", byte);   
        }
        println!("  {}", &s[1]);
    } else {
        println!("Usage: {} FILE", s[0]);
        return;
   }     
}
