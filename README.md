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

See `examples` for another simple example.
