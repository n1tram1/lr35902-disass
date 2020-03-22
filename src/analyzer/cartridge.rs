#[derive(Debug)]
pub struct Cartridge {
    bytes: Vec<u8>,
}

impl Cartridge {
    pub fn from_path(path: &std::path::Path) -> Result<Cartridge, std::io::Error> {
        Ok(Cartridge {
            bytes: std::fs::read(path)?,
        })
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}
