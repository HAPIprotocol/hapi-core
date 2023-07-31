use ethers::{types::H160, utils::to_checksum as ethers_to_checksum};
use regex::Regex;
use std::{net::TcpStream, thread, time::Duration};

pub fn is_tx_match(value: &serde_json::Value) -> bool {
    Regex::new(r"^0x[0-9a-fA-F]{64}$").unwrap().is_match(
        value
            .get("tx")
            .expect("`tx` key not found")
            .as_str()
            .expect("`tx` is not a string"),
    )
}

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
