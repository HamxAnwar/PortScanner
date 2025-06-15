// This project makes a basic port scanner in Rust.
// Usage: cargo run -- <ip_address>

use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::{self, Duration};
use std::{env, process, thread};

fn scan_port(target: &SocketAddr) {
    let stream = TcpStream::connect_timeout(&target, Duration::from_millis(1));
    match stream {
        Ok(_) => println!("Port {} is open", target.port()),
        Err(_) => (), // The port is closed.
    }
}

/*
I have tried to use peek with stream to remove the ephermal ports but it is seen to be not of any benefit.
Since it requires the connection to send data back and in most cases, the data is not sent back.
For example in case of CUPS port (631) which results in port dropage thought the port is open.
*/

fn main() {
    let ipaddr = match env::args().nth(1) {
        Some(value) => IpAddr::from_str(&value).expect("Please provide correct IP address."),
        None => {
            println!("Wrong command usage.");
            println!("Usage: cargo run -- <ip_address>");
            process::exit(1);
        }
    };

    let mut handles = Vec::new();
    let chunk_size = 6553;
    for i in 0..10 {
        let start = i * chunk_size + 1;
        let end = std::cmp::min((i + 1) * chunk_size, 65535);

        let handle = thread::spawn(move || {
            for port in start..=end {
                scan_port(&SocketAddr::new(ipaddr, port as u16));
            }
            thread::sleep(time::Duration::from_secs(1));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
