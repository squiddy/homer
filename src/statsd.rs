use std::str;

#[derive(Debug)]
pub enum MessageKind {
    Counter,
}

pub struct Message<'a> {
    pub key: &'a str,
    pub value: f64,
    pub kind: MessageKind,
}

pub fn parse_package(buf: &[u8]) -> Vec<Message> {
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