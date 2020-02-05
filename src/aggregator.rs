use crate::statsd::{Message, MessageKind};
use std::collections::HashMap;
use std::fmt;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Debug)]
enum Data {
    Count(f64),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Count(v) => write!(f, "{}", v),
        }
    }
}

struct Metric {
    key: String,
    kind: MessageKind,
    data: Data,
}

impl Metric {
    fn new(key: &str, kind: MessageKind) -> Metric {
        Metric {
            key: key.to_string(),
            kind: kind,
            data: match kind {
                MessageKind::Counter => Data::Count(0.0),
            },
        }
    }

    fn record(&mut self, value: f64) {
        // TODO should I check kind here as well?
        self.data = match self.data {
            Data::Count(v) => Data::Count(v + value),
        };
    }

    fn flush<T: Write>(&mut self, output: &mut T, timestamp: u64) {
        match self.kind {
            MessageKind::Counter => match self.data {
                Data::Count(v) => {
                    write!(output, "{} {} {}\n", self.key, v, timestamp);
                    self.data = Data::Count(0.0);
                }
            },
        }
    }
}

pub struct Aggregator {
    data: HashMap<String, Metric>,
}

impl Aggregator {
    pub fn new() -> Aggregator {
        Aggregator {
            data: HashMap::new(),
        }
    }

    pub fn handle(&mut self, msg: &Message) {
        match self.data.get_mut(msg.key) {
            Some(m) => {
                // TODO what happens when the metric type doesn't match?
                if msg.kind != m.kind {
                    return;
                }

                m.record(msg.value);
            }
            None => {
                let mut metric = Metric::new(msg.key, msg.kind);
                metric.record(msg.value);
                self.data.insert(msg.key.to_string(), metric);
            }
        }
    }

    pub fn dump(&self) {
        for (key, metric) in &self.data {
            println!("Aggregator stats:");
            println!("{} ({:?}): {}", key, metric.kind, metric.data);
            println!("");
        }
    }

    pub fn flush<T: Write>(&mut self, output: &mut T) {
        let start = SystemTime::now();
        let unix = start.duration_since(UNIX_EPOCH).unwrap();

        if self.data.is_empty() {
            return;
        }

        println!("Flushing {} metrics...", self.data.len());
        for metric in self.data.values_mut() {
            metric.flush(output, unix.as_secs());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_record() {
        let mut metric = Metric::new("test.key", MessageKind::Counter);
        assert_eq!(metric.data, Data::Count(0.0));

        metric.record(42.0);
        metric.record(-20.0);
        assert_eq!(metric.data, Data::Count(22.0));
    }

    #[test]
    fn counter_flush() {
        let mut metric = Metric::new("test.key", MessageKind::Counter);
        metric.record(23.45);

        let mut output = vec![];
        metric.flush(&mut output, 12345);

        assert_eq!(output, "test.key 23.45 12345\n".as_bytes());
        assert_eq!(metric.data, Data::Count(0.0));
    }
}
