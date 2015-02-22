pub use self::parser::parse;

peg! parser(r##"
use std::num::from_str_radix;
use std::char;
use std::str;
use super::super::*;
use semver::VersionReq;
use semver;

#[pub]
parse -> Puppetfile
  = __ forge:forge __ modules:modules
  { Puppetfile{ forge: forge, modules: modules } }

forge -> String
  = "forge" __ url:string { url }

modules -> Vec<Module>
  = module*

module -> Module
  = "mod" __ name:string __ ("," __)? info:module_info __
  { Module { name: name, info: info } }

module_info -> Vec<ModuleInfo>
  = i:((version / info_hash) ** ("," __)) { i }

version -> ModuleInfo
  = version:string __ {
    if semver::Version::parse(&version).is_ok() {
        ModuleInfo::Version(VersionReq::parse(&format!("={}", version)).unwrap())
    } else {
        ModuleInfo::Version(VersionReq::parse(&version).unwrap())
    }
}

info_hash -> ModuleInfo
  = key:symbol __ "=>" __ value:string __ { ModuleInfo::Info(key, value) }

symbol -> String
  = ":" i:identifier { i }

identifier -> String
  = chars:((letter / "_") (letter / digit / "_")* {match_str.to_string()}) __ { chars }

string -> String
  = string:(doubleQuotedString / singleQuotedString) __ { string }

doubleQuotedString -> String
  = '"' s:doubleQuotedCharacter* '"' { s.into_iter().collect() }

doubleQuotedCharacter -> char
  = simpleDoubleQuotedCharacter
  / simpleEscapeSequence
  / zeroEscapeSequence
  / hexEscapeSequence
  / unicodeEscapeSequence
  / eolEscapeSequence

simpleDoubleQuotedCharacter -> char
  = !('"' / "\\" / eolChar) . { match_str.char_at(0) }

singleQuotedString -> String
  = "'" s:singleQuotedCharacter* "'" { s.into_iter().collect() }

singleQuotedCharacter -> char
  = simpleSingleQuotedCharacter
  / simpleEscapeSequence
  / zeroEscapeSequence
  / hexEscapeSequence
  / unicodeEscapeSequence
  / eolEscapeSequence

simpleSingleQuotedCharacter -> char
  = !("'" / "\\" / eolChar) . { match_str.char_at(0) }

simpleEscapeSequence -> char
  = "\\" !(digit / "x" / "u" / eolChar) . {
      match match_str.char_at(1) {
        //'b' => '\b',
        //'f' => '\f',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        //'v' => '\v',
         x  => x
      }
    }

zeroEscapeSequence -> char
  = "\\0" !digit { 0u8 as char }

hexEscapeSequence -> char
  = "\\x" value:(hexDigit hexDigit { from_str_radix::<u32>(match_str, 16) }) {
      char::from_u32(value.unwrap() as u32).unwrap()
    }

unicodeEscapeSequence -> char
  = "\\u" value:(hexDigit hexDigit hexDigit hexDigit { from_str_radix::<u32>(match_str, 16)}) {
      char::from_u32(value.unwrap() as u32).unwrap()
    }

eolEscapeSequence -> char
  = "\\" eol:eol { '\n' }

digit
  = [0-9]

hexDigit
  = [0-9a-fA-F]

letter
  = lowerCaseLetter
  / upperCaseLetter

lowerCaseLetter
  = [a-z]

upperCaseLetter
  = [A-Z]

__ = (whitespace / eol / comment)*

comment
  = "#" (!eolChar .)*

eol
  = "\n"
  / "\r\n"
  / "\r"
  / "\u{2028}"
  / "\u{2029}"

eolChar
  = [\n\r\u{2028}\u{2029}]

whitespace
  = [ \t\u{00A0}\u{FEFF}\u{1680}\u{180E}\u{2000}-\u{200A}\u{202F}\u{205F}\u{3000}]
"##);
