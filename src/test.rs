use super::{Puppetfile, Module, ModuleInfo, Version};
use semver::{mod, VersionReq};

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
            info: vec![Version(VersionReq::parse("1.0.1").unwrap())]
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

#[test]
fn format() {
    let version = Version(VersionReq::parse("1.0.0").unwrap());
    assert_eq!(String::from_str("= 1.0.0"), format!("{}", version));

    let mod_info = ModuleInfo(
        String::from_str("git"),
        String::from_str("git://github.com/Mayflower/puppet-php.git")
    );
    assert_eq!(
        String::from_str(":git => 'git://github.com/Mayflower/puppet-php.git'"),
        format!("{}", mod_info)
    );

    let module = Module {
        name: String::from_str("mayflower/php"),
        info: vec![version, mod_info]
    };
    assert_eq!(
        String::from_str("mod 'mayflower/php', '= 1.0.0',
  :git => 'git://github.com/Mayflower/puppet-php.git'"),
        format!("{}", module)
    );

    let puppetfile = Puppetfile {
        forge: String::from_str("https://forge.puppetlabs.com"),
        modules: vec![module]
    };
    assert_eq!(
        String::from_str("forge 'https://forge.puppetlabs.com'


mod 'mayflower/php', '= 1.0.0',
  :git => 'git://github.com/Mayflower/puppet-php.git'
"),
        format!("{}", puppetfile)
    );
}

#[test]
fn version_url() {
    let module = Module { name: String::from_str("mayflower/php"), info: vec![] };
    assert_eq!(
        "https://forge.puppetlabs.com/users/mayflower/modules/php/releases/find.json",
        format!(
            "{}",
            module.version_url(
                &"https://forge.puppetlabs.com/".to_string()
            )
        ).as_slice()
    )
}

#[test]
fn user_name_pair() {
    let module = Module { name: String::from_str("mayflower/php"), info: vec![] };
    assert_eq!(module.user_name_pair(), Some(("mayflower", "php")))
}

#[test]
fn forge_version() {
    let module = Module { name: String::from_str("puppetlabs/nginx"), info: vec![] };
    assert_eq!(
        module.forge_version(&"https://forge.puppetlabs.com/".to_string()),
        semver::Version::parse("99.99.99")
    )
}
