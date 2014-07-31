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

#[test]
fn format() {
    let version = Version(String::from_str("1.0.0"));
    assert_eq!(String::from_str("1.0.0"), format!("{}", version));

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
        String::from_str("mod 'mayflower/php', '1.0.0',
  :git => 'git://github.com/Mayflower/puppet-php.git'"),
        format!("{}", module)
    );

    let puppetfile = Puppetfile {
        forge: String::from_str("https://forge.puppetlabs.com"),
        modules: vec![module]
    };
    assert_eq!(
        String::from_str("forge 'https://forge.puppetlabs.com'


mod 'mayflower/php', '1.0.0',
  :git => 'git://github.com/Mayflower/puppet-php.git'
"),
        format!("{}", puppetfile)
    );
}
