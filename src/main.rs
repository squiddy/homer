#![feature(test)]

use std::net::{TcpStream, UdpSocket};

mod aggregator;
mod config;
mod statsd;

fn main() {
    let cfg = config::Config::load("config.toml").unwrap();

    let mut carbon = TcpStream::connect(cfg.carbon_addr).unwrap();
    let mut aggregator = aggregator::Aggregator::new();

    let socket = UdpSocket::bind(cfg.statsd_addr).unwrap();

    loop {
        let mut buf = [0; 8192];
        let amount = socket.recv(&mut buf).unwrap();

        let messages = statsd::parse_package(&buf[..amount]);
        for m in messages {
            println!("message key={} value={} kind={:?}", m.key, m.value, m.kind);
            aggregator.handle(&m);
            aggregator.flush(&mut carbon);
        }
    }
}
