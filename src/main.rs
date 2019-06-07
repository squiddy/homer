use std::net::{UdpSocket};
use std::str;

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
    let socket = UdpSocket::bind("127.0.0.1:8125").unwrap();

    loop {
        let mut buf = [0; 8192];
        let amount = socket.recv(&mut buf).unwrap();
        let metrics = parse_package(&buf[..amount]);
        for m in metrics {
            println!("metric key={} value={} kind={:?}", m.key, m.value, m.kind);
        }
    }
}