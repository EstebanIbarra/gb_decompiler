pub mod gb;
pub mod gba;
pub mod raw;

use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct OpcodeEntry {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub length: u8,
    pub cycles: &'static [u8],
    pub operands: &'static [Operand],
    pub immediate: bool,
    pub flags: Flags,
}

#[derive(Debug)]
pub struct Flags {
    pub z: FlagAction,
    pub n: FlagAction,
    pub h: FlagAction,
    pub c: FlagAction,
}

#[derive(Debug)]
pub enum FlagAction {
    Unaffected,
    Clear,
    Set,
    CopyZ,
    CopyN,
    CopyH,
    CopyC,
}

impl From<String> for FlagAction {
    fn from(s: String) -> FlagAction {
        match s.as_str() {
            "-" => FlagAction::Unaffected,
            "0" => FlagAction::Clear,
            "1" => FlagAction::Set,
            "Z" => FlagAction::CopyZ,
            "N" => FlagAction::CopyN,
            "H" => FlagAction::CopyH,
            "C" => FlagAction::CopyC,
            other => panic!("Unknow flag action: {other}"),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone, PartialEq))]
pub enum Operand {
    Register(&'static str),
    Immediate8,
    Immediate16,
    Offset8,
    Pointer(&'static str),
    IncrementRegister(&'static str),
    DecrementRegister(&'static str),
    Literal(u8),
}

pub fn generate(gb_opcodes: &str, gb_length: usize, _arm_opcodes: &str, _arm_length: usize) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("opcodes.rs");
    let code = String::from(
        r#"pub mod gb;
"#,
    );
    fs::write(dest_path, code).expect("Unable to write generated code");
    gb::generate(gb_opcodes, gb_length, &out_dir);
}
