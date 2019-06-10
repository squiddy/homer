use serde::Deserialize;
use std::error::Error;
use std::io::{BufReader, Read};
use std::net::{SocketAddr};
use std::str;
use std::fs::File;

#[derive(Deserialize)]
pub struct Config {
    pub statsd_addr: SocketAddr,
    pub carbon_addr: SocketAddr,
    pub flush_interval: u64
}

impl Config {
    pub fn load(filename: &str) -> Result<Config, Box<Error>> {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents)?)
    }
}