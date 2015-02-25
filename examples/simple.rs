#![feature(env, fs, io, path)]

extern crate puppetfile;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str;
use puppetfile::Puppetfile;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut puppetfile_contents = String::new();
    File::open(&Path::new(&args[1])).unwrap().read_to_string(&mut puppetfile_contents);
    let puppetfile = Puppetfile::parse(&puppetfile_contents).unwrap_or(
        Puppetfile {
            forge: "https://forge.puppetlabs.com".to_string(),
            modules: vec![]
        }
    );

    println!("{}", puppetfile);
}
