use codegen::opcodes::generate;
use std::fs;

fn main() {
    let gb_opcodes =
        fs::read_to_string("data/gb_opcodes.json").expect("Failed to read GB opcode JSON");
    let arm_opcodes =
        fs::read_to_string("data/arm_thumb_opcodes.json").expect("Failed to read ARM opcode JSON");
    generate(&gb_opcodes, 256, &arm_opcodes, 0);
}
