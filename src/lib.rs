//! This library parses a Puppetfile

#![crate_name = "puppetfile"]
#![deny(missing_doc)]
#![feature(globs)]

use std::fmt;

mod puppetfile_parser;

/// This represents a Puppetfile
#[deriving(PartialEq)]
#[experimental]
pub struct Puppetfile {
    /// The forge URL
    pub forge: String,
    /// All Modules contained in the Puppetfile
    pub modules: Vec<Module>
}

impl Puppetfile {
    /// Try parsing the contents of a Puppetfile into a Puppetfile struct
    pub fn parse(contents: &str) -> Result<Puppetfile, String> {
        puppetfile_parser::parse(contents)
    }
}
impl fmt::Show for Puppetfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = write!(f, "forge '{}'\n\n", self.forge);
        self.modules.iter().fold(res, |prev_res, module| { prev_res.and(write!(f, "\n{}\n", module)) })
    }
}


/// The representation of a puppet module
#[deriving(PartialEq)]
#[experimental]
pub struct Module {
    /// Name of the module
    pub name: String,
    /// More information about the module
    pub info: Vec<ModuleInfo>
}

impl fmt::Show for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = write!(f, "mod '{}'", self.name);
        self.info.iter().fold(res, |prev_res, mod_info| {
            match *mod_info {
                Version(..) => prev_res.and(write!(f, ", '{}'", mod_info)),
                ModuleInfo(..) => prev_res.and(write!(f, ",\n  {}", mod_info)),
            }
        })
    }
}


/// Further Information on Puppet Modules
#[deriving(PartialEq)]
pub enum ModuleInfo {
    /// Version as String
    Version(String),
    /// Key Value based Information
    ModuleInfo(String, String)
}

impl fmt::Show for ModuleInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Version(ref v) => write!(f, "{}", v),
            ModuleInfo(ref k, ref v) => write!(f, ":{} => '{}'", k, v)
        }
    }
}

#[cfg(test)]
mod test;
