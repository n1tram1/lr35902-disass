mod cartridge;
mod instruction;
mod disassembler;

use cartridge::Cartridge;
use instruction::Instruction;
use disassembler::Disassembler;

#[derive(Debug)]
pub struct Analyzer<'a> {
    path: &'a std::path::Path,
    cartridge: cartridge::Cartridge,
}

impl<'a> Analyzer<'a> {
    pub fn from_path(path: &std::path::Path) -> Result<Analyzer, std::io::Error> {
        Ok(Analyzer {
            path,
            cartridge: Cartridge::from_path(path)?,
        })
    }

    pub fn disassemble(&self) -> Vec<Instruction> {
        Disassembler::disassemble(self.cartridge.get_bytes())
    }
}
