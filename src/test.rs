use super::{Puppetfile, Module, ModuleInfo};
use semver::{self, VersionReq};

#[test]
fn empty_file() {
    let puppetfile = Puppetfile::parse(r##"forge "https://forge.puppetlabs.com""##);
    assert!(puppetfile.is_ok());
    let parsed = puppetfile.unwrap();
    assert_eq!(
        "https://forge.puppetlabs.com",
        parsed.forge
    );
    let expected: Vec<Module> = vec![];
    assert_eq!(
        expected,
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
        "https://forge.puppetlabs.com",
        parsed.forge
    );
    assert_eq!(
        Module { name: "mayflower/php".to_string(), info: vec![] },
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
        "https://forge.puppetlabs.com",
        parsed.forge
    );
    assert_eq!(
        Module {
            name: "mayflower/php".to_string(),
            info: vec![ModuleInfo::Version(VersionReq::parse("= 1.0.1").unwrap())]
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
        "https://forge.puppetlabs.com",
        parsed.forge
    );
    assert_eq!(
        Module {
            name: "mayflower/php".to_string(),
            info: vec![
                ModuleInfo::Info("git".to_string(),
                                 "git://github.com/Mayflower/puppet-php.git".to_string())
            ]
        },
        parsed.modules[0]
    );
}

#[test]
fn format() {
    let version = ModuleInfo::Version(VersionReq::parse("= 1.0.0").unwrap());
    assert_eq!("= 1.0.0".to_string(), format!("{}", version));

    let mod_info = ModuleInfo::Info(
        "git".to_string(),
        "git://github.com/Mayflower/puppet-php.git".to_string()
    );
    assert_eq!(
        ":git => 'git://github.com/Mayflower/puppet-php.git'",
        format!("{}", mod_info)
    );

    let module = Module {
        name: "mayflower/php".to_string(),
        info: vec![version, mod_info]
    };
    assert_eq!(
        "mod 'mayflower/php', '= 1.0.0',
  :git => 'git://github.com/Mayflower/puppet-php.git'",
        format!("{}", module)
    );

    let puppetfile = Puppetfile {
        forge: "https://forge.puppetlabs.com".to_string(),
        modules: vec![module]
    };
    assert_eq!(
        "forge 'https://forge.puppetlabs.com'


mod 'mayflower/php', '= 1.0.0',
  :git => 'git://github.com/Mayflower/puppet-php.git'
",
        format!("{}", puppetfile)
    );
}

#[test]
fn version_url() {
    let module = Module { name: "mayflower/php".to_string(), info: vec![] };
    assert_eq!(
        "https://forge.puppetlabs.com/users/mayflower/modules/php/releases/find.json".to_string(),
        module.version_url("https://forge.puppetlabs.com/").unwrap()
    )
}

#[test]
fn user_name_pair() {
    let module = Module { name: "mayflower/php".to_string(), info: vec![] };
    assert_eq!(module.user_name_pair(), Some(("mayflower", "php")))
}

#[test]
fn forge_version() {
    let module = Module { name: "puppetlabs/nginx".to_string(), info: vec![] };
    assert_eq!(
        module.forge_version(&"https://forge.puppetlabs.com/".to_string()).unwrap(),
        semver::Version::parse("99.99.99").unwrap()
    )
}
