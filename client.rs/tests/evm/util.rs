use ethers::{types::H160, utils::to_checksum as ethers_to_checksum};
use std::{net::TcpStream, thread, time::Duration};

pub fn to_checksum(value: &str) -> String {
    ethers_to_checksum(&value.parse::<H160>().expect("invalid address"), None)
}

pub fn wait_for_port(port: u16) {
    let addr = format!("localhost:{}", port);
    loop {
        thread::sleep(Duration::from_secs(1));
        match TcpStream::connect(&addr) {
            Ok(_) => {
                println!("Successfully connected to the server at {}", &addr);
                break;
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
}
