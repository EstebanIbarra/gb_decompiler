use super::FlagAction;
use super::raw::{RawOpcode, RawOperand};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Root {
    pub unprefixed: HashMap<String, RawOpcode>,
    pub cbprefixed: HashMap<String, RawOpcode>,
}

pub fn generate(data: &str, length: usize, out_dir: &str) {
    let raw_map: Root = serde_json::from_str(data).expect("Failed to read GB opcode JSON");
    if raw_map.unprefixed.len() != length || raw_map.cbprefixed.len() != length {
        panic!(
            r#"Incomplete Opcodes:
    GB Unprefixed Opcodes: {}
    GB CB-Prefixed Opcodes: {}
"#,
            raw_map.unprefixed.len(),
            raw_map.cbprefixed.len()
        );
    }
    let dest_path = Path::new(out_dir).join("gb.rs");
    let mut code = String::new();

    code.push_str(
        r#"use codegen::opcodes::*;

pub const OPCODES: [OpcodeEntry; 256] = [
"#,
    );
    code = map_opcodes(code, raw_map.unprefixed);
    code.push_str("];\n\n");
    code.push_str("pub const CB_OPCODES: [OpcodeEntry; 256] = [\n");
    code = map_opcodes(code, raw_map.cbprefixed);
    code.push_str("];\n");
    fs::write(dest_path, code).expect("Unable to write generated code");
}

pub fn map_opcodes(mut code: String, opcodes: HashMap<String, RawOpcode>) -> String {
    for idx in 0u8..=255 {
        let hex = format!("0x{idx:02X}");
        if let Some(opcode) = opcodes.get(&hex) {
            code.push_str(&format!(
                r#"    OpcodeEntry {{
        opcode: {},
        mnemonic: "{}",
        length: {},
        cycles: &[
"#,
                hex, opcode.mnemonic, opcode.length
            ));
            for cycle in &opcode.cycles {
                code.push_str(&format!("            {cycle},\n"));
            }
            code.push_str("        ],\n");
            code.push_str("        operands: &[\n");
            for operand in &opcode.operands {
                code.push_str(&format!(
                    "            Operand::{},\n",
                    render_operand(operand)
                ));
            }
            code.push_str(&format!(
                r#"        ],
        immediate: {},
        flags: Flags {{
            z: FlagAction::{:?},
            n: FlagAction::{:?},
            h: FlagAction::{:?},
            c: FlagAction::{:?},
        }},
    }},
"#,
                opcode.immediate,
                FlagAction::from(opcode.flags.z.clone()),
                FlagAction::from(opcode.flags.n.clone()),
                FlagAction::from(opcode.flags.h.clone()),
                FlagAction::from(opcode.flags.c.clone()),
            ));
        } else {
            code.push_str("    None,\n");
        }
    }
    code
}

pub fn render_operand(raw: &RawOperand) -> String {
    if raw.increment {
        return format!(r#"IncrementRegister("{}")"#, raw.name);
    }
    if raw.decrement {
        return format!(r#"DecrementRegister("{}")"#, raw.name);
    }

    if let Some(hex) = raw.name.strip_prefix('$') {
        let b = u8::from_str_radix(hex, 16)
            .unwrap_or_else(|_| panic!("Invalid hex literal in operand: {}", &raw.name));
        return format!("Literal({b})");
    }

    if raw.immediate && raw.name.chars().all(|c| c.is_ascii_digit()) {
        let b = raw.name.parse::<u8>().unwrap();
        return format!("Literal({b})");
    }

    if raw.name == "d8" || raw.name == "e8" {
        return "Offset8".to_string();
    }

    if raw.name.starts_with('n') {
        match raw.length.unwrap_or(1) {
            1 => return "Immediate8".to_string(),
            2 => return "Immediate16".to_string(),
            n => panic!("Unexpected immediate size {} for {}", n, raw.name),
        }
    }

    if raw.name == "a8" || raw.name == "a16" {
        return format!(r#"Pointer("{}")"#, raw.name);
    }

    format!(r#"Register("{}")"#, raw.name)
}
