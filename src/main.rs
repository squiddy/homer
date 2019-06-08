use std::io::Write;
use std::net::{TcpStream, UdpSocket};
use std::time::{SystemTime, UNIX_EPOCH};

mod config;
mod statsd;

fn main() {
    let cfg = config::Config::load("config.toml").unwrap();

    let mut carbon = TcpStream::connect(cfg.carbon_addr).unwrap();
    let socket = UdpSocket::bind(cfg.statsd_addr).unwrap();

    loop {
        let mut buf = [0; 8192];
        let amount = socket.recv(&mut buf).unwrap();
        let metrics = statsd::parse_package(&buf[..amount]);

        let start = SystemTime::now();
        let unix = start.duration_since(UNIX_EPOCH).unwrap();
        for m in metrics {
            println!("metric key={} value={} kind={:?}", m.key, m.value, m.kind);
            write!(carbon, "{} {} {}\n", m.key, m.value, unix.as_secs()).unwrap();
        }
    }
}