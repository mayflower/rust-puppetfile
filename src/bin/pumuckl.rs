extern crate puppetfile;
extern crate http;
extern crate serialize;
extern crate semver;

use std::io::File;
use std::os;
use std::str;
use std::sync::Future;
use semver::version::{Version, ParseError};
use puppetfile::Puppetfile;


fn main() {
    let args = os::args();
    let file_raw_bytes = File::open(&Path::new(args[1].as_slice())).read_to_end().unwrap();
    let puppetfile_contents = str::from_utf8(file_raw_bytes.as_slice()).unwrap();
    let puppetfile = Puppetfile::parse(puppetfile_contents).unwrap();

    let modules = puppetfile.modules.clone();
    let mut version_ftrs: Vec<Future<(String, Result<Version, ParseError>)>> = modules.move_iter().filter(
        |m| m.user_name_pair().is_some()
    ).map(|m| {
        let forge_url = puppetfile.forge.clone();
        Future::spawn(proc() {
            (m.name.clone(), m.forge_version(forge_url))
        })
    }).collect();

    let versions: Vec<(String, Version)> = version_ftrs.mut_iter().map(
        |ftr| ftr.get()
    ).filter_map(
        |tpl| match tpl {
            (name, Ok(version)) => Some((name, version)),
            (_, Err(_)) => None
        }
    ).collect();

    let mut modules_to_check = puppetfile.modules.iter().filter(
        |m| m.user_name_pair().is_some() && m.version().is_some()
    );

    for module in modules_to_check {
        for &(ref name, ref version) in versions.iter() {
            if module.name == *name && module.version().is_some() &&
                    module.version().unwrap() != *version {
                println!("{}: {} != {}", module.name, module.version().unwrap(), version)
            }
        }
    }
}
