//! This library parses a Puppetfile
#![deny(missing_docs)]

extern crate hyper;
extern crate semver;
extern crate rustc_serialize;

use std::error::Error;
use std::fmt;
use std::io::{self, Read};

use hyper::Client;
use rustc_serialize::json;
use semver::VersionReq;

use ErrorKind::*;

mod grammar;

#[cfg(test)]
mod test;

/// This represents a Puppetfile
#[derive(PartialEq, Clone, Debug)]
pub struct Puppetfile {
    /// The forge URL
    pub forge: String,
    /// All Modules contained in the Puppetfile
    pub modules: Vec<Module>
}

impl Puppetfile {
    /// Try parsing the contents of a Puppetfile into a Puppetfile struct
    pub fn parse(contents: &str) -> Result<Puppetfile, PuppetfileError> {
        Ok(try!(grammar::parse(contents)))
    }
}
impl fmt::Display for Puppetfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = write!(f, "forge '{}'\n\n", self.forge);
        self.modules.iter().fold(res, |prev_res, module| { prev_res.and(write!(f, "\n{}\n", module)) })
    }
}


/// The representation of a puppet module
#[derive(PartialEq, Clone, Debug)]
pub struct Module {
    /// Name of the module
    pub name: String,
    /// More information about the module
    pub info: Vec<ModuleInfo>
}

#[derive(RustcDecodable)]
struct ForgeVersionResponse {
    version: String
}

/// represents the type of error of a PuppetfileError
#[derive(Debug)]
pub enum ErrorKind {
    /// an HTTP error
    HttpError(hyper::Error),
    /// an IO error
    IoError(io::Error),
    /// an error while parsing the version
    SemverError(semver::ParseError),
    /// an error while parsing JSON
    JsonError(json::DecoderError),
    /// an error while building the forge URL
    UrlBuilding,
    /// an HTTP error
    ParseError(grammar::ParseError),
}
/// represents an error while checking the version published on the forge
#[derive(Debug)]
pub struct PuppetfileError {
    /// type of the error
    pub kind: ErrorKind,
    /// short description
    pub desc: String,
}

impl fmt::Display for PuppetfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<hyper::Error> for PuppetfileError {
    fn from(err: hyper::Error) -> PuppetfileError {
        From::from((HttpError(err), "an HTTP error occured".to_string()))
    }
}

impl From<io::Error> for PuppetfileError {
    fn from(err: io::Error) -> PuppetfileError {
        From::from((IoError(err), "an IO error occured".to_string()))
    }
}

impl From<semver::ParseError> for PuppetfileError {
    fn from(err: semver::ParseError) -> PuppetfileError {
        From::from((SemverError(err), "an invalid version was given".to_string()))
    }
}

impl From<grammar::ParseError> for PuppetfileError {
    fn from(err: grammar::ParseError) -> PuppetfileError {
        From::from((ParseError(err), "could not parse the Puppetfile".to_string()))
    }
}

impl From<json::DecoderError> for PuppetfileError {
    fn from(err: json::DecoderError) -> PuppetfileError {
        From::from((JsonError(err), "an error occured while decoding JSON".to_string()))
    }
}

impl From<(ErrorKind, String)> for PuppetfileError {
    fn from((kind, desc): (ErrorKind, String)) -> PuppetfileError {
        PuppetfileError {
            kind: kind,
            desc: desc,
        }
    }
}


impl Error for PuppetfileError {
    fn description(&self) -> &str {
        &self.desc
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind {
            JsonError(ref err) => Some(err as &Error),
            HttpError(ref err) => Some(err as &Error),
            IoError(ref err) => Some(err as &Error),
            SemverError(ref err) => Some(err as &Error),
            ParseError(ref err) => Some(err as &Error),
            _ => None
        }
    }
}

impl Module {
    /// The current version of the module returned from the forge API
    pub fn forge_version(&self, forge_url: &str) -> Result<semver::Version, PuppetfileError> {
        let url = try!(self.version_url(forge_url));
        let client = Client::new();
        let mut response = try!(client.get(&*url).send());
        let mut response_string = String::new();
        try!(response.read_to_string(&mut response_string));
        let version_struct: ForgeVersionResponse = try!(json::decode(&response_string));
        let version = try!(semver::Version::parse(&version_struct.version));

        Ok(version)
    }

    /// Builds the URL for the forge API for fetching the version
    pub fn version_url(&self, forge_url: &str) -> Result<String, PuppetfileError> {
        let stripped_url = match forge_url.ends_with("/") {
            true => &forge_url[..forge_url.len() - 1],
            _    => &forge_url[..]
        };
        let (user, mod_name) = match self.user_name_pair() {
            Some((user, mod_name)) => (user, mod_name),
            None => return Err(From::from((UrlBuilding, "Could not build url".to_string())))
        };

        Ok(format!("{}/users/{}/modules/{}/releases/find.json", stripped_url, user, mod_name))
    }

    /// Returns user and module name from 'user/mod_name'
    pub fn user_name_pair(&self) -> Option<(&str, &str)> {
        if self.name.contains("/") {
            let mut parts = self.name.split('/');
            Some((parts.next().unwrap(), parts.next().unwrap()))
        } else {
            None
        }
    }

    /// Returns the version if specified
    pub fn version(&self) -> Option<&VersionReq> {
        for info in self.info.iter() {
            match *info {
                ModuleInfo::Version(ref v) => return Some(v),
                ModuleInfo::Info(..) => ()
            }
        }
        None
    }
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = write!(f, "mod '{}'", self.name);
        self.info.iter().fold(res, |prev_res, mod_info| {
            match *mod_info {
                ModuleInfo::Version(..) => prev_res.and(write!(f, ", '{}'", mod_info)),
                ModuleInfo::Info(..) => prev_res.and(write!(f, ",\n  {}", mod_info)),
            }
        })
    }
}


/// Further Information on Puppet Modules
#[derive(PartialEq, Clone, Debug)]
pub enum ModuleInfo {
    /// Version as String
    Version(VersionReq),
    /// Key Value based Information
    Info(String, String)
}
impl ModuleInfo {
    /// Returns `true` if the option is a `Version` value
    pub fn is_version(&self) -> bool {
        match *self {
            ModuleInfo::Version(..)    => true,
            ModuleInfo::Info(..) => false
        }
    }
}

impl fmt::Display for ModuleInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModuleInfo::Version(ref v) => write!(f, "{}", v),
            ModuleInfo::Info(ref k, ref v) => write!(f, ":{} => '{}'", k, v)
        }
    }
}

