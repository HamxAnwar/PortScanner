// This project makes a basic port scanner in Rust.
// Usage: cargo run -- <ip_address>

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::time::Duration;
use std::{env, process, thread};

fn scan_port(target: &SocketAddr) {
    let stream = TcpStream::connect_timeout(&target, Duration::from_millis(1000));
    match stream {
        Ok(stream) => println!("Port {} is open", target.port()),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn main() {
    // let ipaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80);
    for port in 1..65536 {
        scan_port(&SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(192, 168, 16, 119)),
            port as u16,
        ));
    }
}
