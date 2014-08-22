rust-puppetfile
===============

[![Build Status](https://travis-ci.org/Mayflower/rust-puppetfile.svg?branch=master)](https://travis-ci.org/Mayflower/rust-puppetfile)

Small Puppetfile Parser

## Usage
```rust
let puppetfile = Puppetfile::parse(r##"forge "https://forge.puppetlabs.com"

mod 'mayflower/php', '1.0.1'
        "##);
```

See `examples` for another simple example or `src/bin/pumuckl.rs` as real use case

## Documentation
At [rust-ci](http://www.rust-ci.org/Mayflower/rust-puppetfile/doc/puppetfile/)

## Pumuckl
**Pumuckl** checks a Puppetfile for newer versions of modules on the puppet forge
Simply run it with:
```
cargo build
./target/pumuckl path to Puppetfile
```
