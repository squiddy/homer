use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct Config {
    statsd_addr: SocketAddr,
    carbon_addr: SocketAddr,
}

#[derive(Debug)]
enum MessageKind {
    Counter,
}

struct Message<'a> {
    key: &'a str,
    value: f64,
    kind: MessageKind,
}

fn parse_package(buf: &[u8]) -> Vec<Message> {
    // Parse an incoming statsd packet and return a list of metrics with their values.
    // TODO: Make it more readable, error-proof and faster.

    let mut metrics = Vec::new();

    unsafe {
        let string = str::from_utf8_unchecked(buf);
        for line in string.lines() {
            let mut bits = line.split('|');
            let part1 = bits.next().unwrap();
            let kind = match bits.next().unwrap() {
                "c" => MessageKind::Counter,
                _ => continue,
            };
            let mut bits = part1.split(':');

            metrics.push(Message {
                key: bits.next().unwrap(),
                value: bits.next().unwrap().parse().unwrap(),
                kind: kind,
            });
        }
    }

    return metrics;
}

fn main() {
    let file = File::open("config.toml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();

    let mut carbon = TcpStream::connect(config.carbon_addr).unwrap();
    let socket = UdpSocket::bind(config.statsd_addr).unwrap();

    loop {
        let mut buf = [0; 8192];
        let amount = socket.recv(&mut buf).unwrap();
        let metrics = parse_package(&buf[..amount]);

        let start = SystemTime::now();
        let unix = start.duration_since(UNIX_EPOCH).unwrap();
        for m in metrics {
            println!("metric key={} value={} kind={:?}", m.key, m.value, m.kind);
            write!(carbon, "{} {} {}\n", m.key, m.value, unix.as_secs()).unwrap();
        }
    }
}
