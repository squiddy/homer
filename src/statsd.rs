use std::str;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MessageKind {
    Counter,
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{black_box, Bencher};

    #[test]
    fn parses_single_metric_packages() {
        let input = "this.is.a.counter:1|c";
        let result = parse_package(input.as_bytes());
        assert_eq!(
            result,
            vec![Message {
                key: "this.is.a.counter",
                value: 1.0,
                kind: MessageKind::Counter
            }]
        );
    }

    #[test]
    fn parses_multi_metric_packages() {
        let input = "this.is.a.counter:1|c\nanother.counter.here:34.12|c\n";
        let result = parse_package(input.as_bytes());
        assert_eq!(
            result,
            vec![
                Message {
                    key: "this.is.a.counter",
                    value: 1.0,
                    kind: MessageKind::Counter
                },
                Message {
                    key: "another.counter.here",
                    value: 34.12,
                    kind: MessageKind::Counter
                }
            ]
        );
    }

    #[test]
    fn ignores_unknown_metric_types() {
        let input = "this.is.something:1|X\nthis.is.a.counter:34.12|c\n";
        let result = parse_package(input.as_bytes());
        assert_eq!(
            result,
            vec![Message {
                key: "this.is.a.counter",
                value: 34.12,
                kind: MessageKind::Counter
            }]
        );
    }

    #[bench]
    pub fn bench_parsing(b: &mut Bencher) {
        let input = "this.is.a.counter:1|c";
        b.iter(|| {
            black_box(parse_package(input.as_bytes()));
        })
    }
}
