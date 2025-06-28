use super::*;
use crate::test_utils::load_roms;

#[test]
pub fn test_detect_rom() {
    let roms = load_roms();
    for rom in roms {
        let expected = match rom.extension.as_str() {
            "gba" => RomType::GBA,
            _ => match rom.data[0x143] {
                0xC0 | 0x80 => RomType::GBC,
                0x00 => RomType::GB,
                _ => RomType::Unknown,
            },
        };
        let rom_type = RomType::from(&rom.data[..]);
        assert_eq!(rom_type, expected);
    }
}
