//! This library parses a Puppetfile

#![crate_name = "puppetfile"]
#![deny(missing_doc)]
#![feature(globs)]

extern crate http;
extern crate serialize;
extern crate semver;
extern crate url;

use std::fmt;
use serialize::json;
use semver::VersionReq;
use http::client::RequestWriter;
use http::method::Get;
use url::Url;

mod puppetfile_parser;

#[cfg(test)]
mod test;

/// This represents a Puppetfile
#[deriving(PartialEq, Clone)]
#[experimental]
pub struct Puppetfile {
    /// The forge URL
    pub forge: String,
    /// All Modules contained in the Puppetfile
    pub modules: Vec<Module>
}

#[experimental]
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
#[deriving(PartialEq, Clone)]
#[experimental]
pub struct Module {
    /// Name of the module
    pub name: String,
    /// More information about the module
    pub info: Vec<PuppetModuleInfo>
}

#[deriving(Decodable)]
struct ForgeVersionResponse {
    version: String
}

/// represents an error while checking the version published on the forge
#[deriving(Clone, Show)]
pub struct ForgeVersionError(String);

#[experimental]
impl Module {
    /// The current version of the module returned from the forge API
    pub fn forge_version(&self, forge_url: &String) -> Result<semver::Version, ForgeVersionError> {
        let url = try!(self.version_url(forge_url));
        let request: RequestWriter = try!(RequestWriter::new(Get, url).map_err(|err|
            ForgeVersionError(format!("{}", err))
        ));
        let mut response = try!(request.read_response().map_err(|(_, err)|
            ForgeVersionError(format!("{}", err))
        ));
        let response_string = try!(response.read_to_string().map_err(|err|
            ForgeVersionError(format!("{}", err))
        ));
        let version_struct: ForgeVersionResponse = try!(json::decode(response_string.as_slice()).map_err(|err|
            ForgeVersionError(format!("{}", err))
        ));
        semver::Version::parse(version_struct.version.as_slice()).map_err(|err|
            ForgeVersionError(format!("{}", err))
        )
    }

    /// Builds the URL for the forge API for fetching the version
    pub fn version_url(&self, forge_url: &String) -> Result<Url, ForgeVersionError> {
        let stripped_url = match forge_url.as_slice().ends_with("/") {
            true => forge_url.as_slice().slice_to(forge_url.len() - 1),
            _    => forge_url.as_slice()
        };
        let (user, mod_name) = match self.user_name_pair() {
            Some((user, mod_name)) => (user, mod_name),
            None => return Err(ForgeVersionError("Could not build url".to_string()))
        };
        Url::parse(format!("{}/users/{}/modules/{}/releases/find.json", stripped_url, user, mod_name).as_slice()).map_err(|err|
            ForgeVersionError(format!("{}", err))
        )
    }

    /// Returns user and module name from 'user/mod_name'
    pub fn user_name_pair(&self) -> Option<(&str, &str)> {
        if self.name.as_slice().contains("/") {
            let mut parts = self.name.as_slice().split('/');
            Some((parts.next().unwrap(), parts.next().unwrap()))
        } else {
            None
        }
    }

    /// Returns the version if specified
    pub fn version(&self) -> Option<&VersionReq> {
        for info in self.info.iter() {
            match *info {
                Version(ref v) => return Some(v),
                ModuleInfo(..) => ()
            }
        }
        None
    }
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
#[deriving(PartialEq, Clone)]
pub enum PuppetModuleInfo {
    /// Version as String
    Version(VersionReq),
    /// Key Value based Information
    ModuleInfo(String, String)
}
impl PuppetModuleInfo {
    /// Returns `true` if the option is a `Version` value
    pub fn is_version(&self) -> bool {
        match *self {
            Version(..)    => true,
            ModuleInfo(..) => false
        }
    }
}

impl fmt::Show for PuppetModuleInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Version(ref v) => write!(f, "{}", v),
            ModuleInfo(ref k, ref v) => write!(f, ":{} => '{}'", k, v)
        }
    }
}

