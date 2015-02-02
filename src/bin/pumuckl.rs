#![feature(slicing_syntax)]

extern crate puppetfile;
extern crate serialize;
extern crate semver;

use std::old_io::File;
use std::os;
use std::str;
use std::sync::Future;
use semver::Version;
use puppetfile::{PuppetfileError, Puppetfile};


fn main() {
    let args = os::args();
    let path = if args.len() == 1 {
        Path::new("Puppetfile")
    } else {
        Path::new(&args[1][])
    };
    let file_raw_bytes = match File::open(&path).read_to_end() {
        Ok(bytes)  => bytes,
        Err(error) => panic!("{}", error)
    };
    let puppetfile_contents = str::from_utf8(&file_raw_bytes[]).unwrap();
    let puppetfile = Puppetfile::parse(puppetfile_contents).unwrap();

    let modules = puppetfile.modules.clone();
    let mut version_ftrs: Vec<Future<(String, Result<Version, PuppetfileError>)>> = modules.into_iter().filter(
        |m| m.user_name_pair().is_some()
    ).map(|m| {
        let forge_url = puppetfile.forge.clone();
        Future::spawn(move || {
            (m.name.clone(), m.forge_version(&forge_url))
        })
    }).collect();

    let versions: Vec<(String, Version)> = version_ftrs.iter_mut().map(
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
                    !module.version().unwrap().matches(version) {
                println!("{}: {} doesn't match {}", module.name, module.version().unwrap(), version)
            }
        }
    }
}
