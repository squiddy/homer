# Homer

Homer is a [statsd](https://github.com/etsy/statsd)-compatible stats
aggregator written in Rust. I'm basing this implementation on github's statsd
implementation [brubeck](https://github.com/github/brubeck).

## Why

To write something in Rust. I've been reading statsd related code lately,
especially a C-implementation, and I'd like to see whether I can make it
shorter and easier to understand for my usecase.

## What's working

* [x] configuration loading
* [ ] statsd
    * [x] receive packets via UDP
    * [x] basic packet parsing (no support for sampling rates)
    * [ ] performance (recvmmsg, multi thread)
* [ ] aggregation
    * [x] counter
    * [ ] gauge
    * [ ] timer
    * [ ] sets (maybe later)
* [ ] carbon
    * [x] plain text protocol
    * [x] periodic flushing
    * [ ] pickle protocol (maybe later)
* [ ] logging
* [ ] proper error handling

## Setup

Get the nightly rust compiler in version 1.36.0 and run `cargo run`.
Configuration is possible by editing the `config.toml` in the working
directory.