#[cfg(test)]
mod tests;

/// RomType enums all the possible GameBoy ROM types.
#[derive(Debug, PartialEq)]
pub enum RomType {
    /// GameBoy
    GB,
    /// GameBoy Color Only
    GBC,
    /// GameBoy Advance
    GBA,
    /// Unknown
    Unknown,
}

impl From<&[u8]> for RomType {
    fn from(rom: &[u8]) -> RomType {
        if rom.len() < 0xB3 {
            return RomType::Unknown;
        }
        if rom[0xB2..0xB4] == [0x96, 0x00] {
            return RomType::GBA;
        }
        match rom[0x143] {
            0xC0 | 0x80 => RomType::GBC,
            0x00 => RomType::GB,
            _ => RomType::Unknown,
        }
    }
}

pub enum Instruction {
    Unknown,
    NOP,
    LD,
    INC,
    DEC,
}
