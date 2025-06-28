use std::fs;
use std::path::Path;

#[cfg(test)]
pub struct RomFile {
    pub filename: String,
    pub extension: String,
    pub length: usize,
    pub data: Vec<u8>,
}

#[cfg(test)]
pub fn load_roms() -> Vec<RomFile> {
    let mut roms = vec![];
    let dir = fs::read_dir("_mock/ROMS").expect("Failed to read directory");
    for entry in dir {
        let entry = entry.expect("Failed to read directory entry");
        let file = entry.file_name();
        let file = Path::new(&file);
        let filename = file
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("")
            .to_string();
        let extension = file
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("")
            .to_string();
        if !extension.contains("gb") {
            continue;
        }
        let file_type = entry.file_type().expect("Failed to get the file type");
        if file_type.is_file() {
            let data = fs::read(entry.path()).expect("Failed to read file");
            let length = data.len();
            roms.push(RomFile {
                filename,
                extension,
                length,
                data,
            });
        }
    }
    roms
}
