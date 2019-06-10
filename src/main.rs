#![feature(test)]

use std::net::{TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod aggregator;
mod config;
mod statsd;

fn main() {
    let cfg = config::Config::load("config.toml").unwrap();

    let mut carbon = TcpStream::connect(cfg.carbon_addr).unwrap();
    let mut aggregator = Arc::new(Mutex::new(aggregator::Aggregator::new()));

    let socket = UdpSocket::bind(cfg.statsd_addr).unwrap();

    {
        let aggregator = Arc::clone(&mut aggregator);
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(cfg.flush_interval));
            aggregator.lock().unwrap().flush(&mut carbon);
        });
    }

    {
        let aggregator = Arc::clone(&mut aggregator);
        loop {
            let mut buf = [0; 8192];
            let amount = socket.recv(&mut buf).unwrap();

            let messages = statsd::parse_package(&buf[..amount]);
            for m in messages {
                println!("message key={} value={} kind={:?}", m.key, m.value, m.kind);
                aggregator.lock().unwrap().handle(&m);
            }
        }
    }
}
