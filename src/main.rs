// This project makes a basic port scanner in Rust.
// Usage: cargo run -- <ip_address>

use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;
use std::{env, process};

fn scan_port(target: &SocketAddr) {
    let stream = TcpStream::connect_timeout(&target, Duration::from_millis(1000));
    match stream {
        Ok(_) => println!("Port {} is open", target.port()),
        Err(_) => (), // The port is closed.
    }
}

fn main() {
    let ipaddr = match env::args().nth(1) {
        Some(value) => IpAddr::from_str(&value).expect("Please provide correct IP address."),
        None => {
            println!("Wrong command usage.");
            println!("Usage: cargo run -- <ip_address>");
            process::exit(1);
        }
    };
    // let ipaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80);
    for port in 1..65536 {
        scan_port(&SocketAddr::new(ipaddr, port as u16));
    }
}
