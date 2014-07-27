//! This library parses a Puppetfile

#![crate_name = "puppetfile"]
#![deny(missing_doc)]
#![feature(globs)]

mod puppetfile_parser;

/// This represents a Puppetfile
#[deriving(Show, PartialEq)]
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

/// The representation of a puppet module
#[deriving(Show, PartialEq)]
pub struct Module {
    /// Name of the module
    pub name: String,
    /// More information about the module
    pub info: Vec<ModuleInfo>
}

/// Further Information on Puppet Modules
#[deriving(Show, PartialEq)]
pub enum ModuleInfo {
    /// Version as String
    Version(String),
    /// Key Value based Information
    ModuleInfo(String, String)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_file() {
        let puppetfile = Puppetfile::parse(r##"forge "https://forge.puppetlabs.com""##);
        assert!(puppetfile.is_ok());
        let parsed = puppetfile.unwrap();
        assert_eq!(
            String::from_str("https://forge.puppetlabs.com"),
            parsed.forge
        );
        assert_eq!(
            vec![],
            parsed.modules
        );
    }

    #[test]
    fn no_version() {
        let puppetfile = Puppetfile::parse(r##"forge "https://forge.puppetlabs.com"

mod 'mayflower/php'
        "##);
        assert!(puppetfile.is_ok());

        let parsed = puppetfile.unwrap();
        assert_eq!(
            String::from_str("https://forge.puppetlabs.com"),
            parsed.forge
        );
        assert_eq!(
            Module { name: String::from_str("mayflower/php"), info: vec![] },
            parsed.modules[0]
        );
    }

    #[test]
    fn git_version() {
        let puppetfile = Puppetfile::parse(r##"forge "https://forge.puppetlabs.com"

mod 'mayflower/php', '1.0.1'
        "##);
        assert!(puppetfile.is_ok());

        let parsed = puppetfile.unwrap();
        assert_eq!(
            String::from_str("https://forge.puppetlabs.com"),
            parsed.forge
        );
        assert_eq!(
            Module {
                name: String::from_str("mayflower/php"),
                info: vec![Version(String::from_str("1.0.1"))]
            },
            parsed.modules[0]
        );
    }

    #[test]
    fn version() {
        let puppetfile = Puppetfile::parse(r##"forge "https://forge.puppetlabs.com"

mod 'mayflower/php',
    :git => 'git://github.com/Mayflower/puppet-php.git'
        "##);
        assert!(puppetfile.is_ok());

        let parsed = puppetfile.unwrap();
        assert_eq!(
            String::from_str("https://forge.puppetlabs.com"),
            parsed.forge
        );
        assert_eq!(
            Module {
                name: String::from_str("mayflower/php"),
                info: vec![
                    ModuleInfo(String::from_str("git"),
                               String::from_str("git://github.com/Mayflower/puppet-php.git"))
                ]
            },
            parsed.modules[0]
        );
    }
}
