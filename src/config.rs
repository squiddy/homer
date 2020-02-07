use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::net::SocketAddr;
use std::str;

#[derive(Deserialize)]
pub struct Config {
    pub statsd_addr: SocketAddr,
    pub carbon_addr: SocketAddr,
    pub flush_interval: u64,
}

impl Config {
    pub fn load(filename: &str) -> Result<Config, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents)?)
    }
}
