use std::{net::TcpStream, thread, time::Duration};

use regex::Regex;

pub fn wait_for_port(port: u16) {
    let addr = format!("localhost:{}", port);
    let mut count = 0;
    loop {
        thread::sleep(Duration::from_secs(1));
        match TcpStream::connect(&addr) {
            Ok(_) => {
                println!("Successfully connected to the server at {}", &addr);
                break;
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
                count += 1;
                if count > 10 {
                    panic!("Failed to connect to the server at {}", &addr);
                }
            }
        }
    }
}

pub fn is_tx_match(value: &serde_json::Value) -> bool {
    Regex::new(r"[0-9a-zA-Z]{43,44}$").unwrap().is_match(
        value
            .get("tx")
            .expect("`tx` key not found")
            .as_str()
            .expect("`tx` is not a string"),
    )
}
