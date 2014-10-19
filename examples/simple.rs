extern crate puppetfile;

use std::io::File;
use std::os;
use std::str;
use puppetfile::Puppetfile;

fn main() {
    let args = os::args();
    let file_raw_bytes = File::open(&Path::new(args[1].as_slice())).read_to_end().unwrap();
    let puppetfile_contents = str::from_utf8(file_raw_bytes.as_slice()).unwrap();
    let puppetfile = Puppetfile::parse(puppetfile_contents).unwrap_or(
        Puppetfile {
            forge: String::from_str("https://forge.puppetlabs.com"),
            modules: vec![]
        }
    );

    println!("{}", puppetfile);
}
