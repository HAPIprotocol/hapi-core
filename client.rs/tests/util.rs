use std::{net::TcpStream, thread, time::Duration};

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
