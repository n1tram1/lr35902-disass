mod cartridge;
mod disassembler;
mod error;
mod instruction;

use cartridge::Cartridge;
use disassembler::Disassembler;
use error::AnalyzerError;
use instruction::Instruction;

#[derive(Debug)]
pub struct Analyzer<'a> {
    path: &'a std::path::Path,
    cartridge: cartridge::Cartridge,
}

impl<'a> Analyzer<'a> {
    pub fn from_path(path: &std::path::Path) -> Result<Analyzer, AnalyzerError> {
        Ok(Analyzer {
            path,
            cartridge: Cartridge::from_path(path)?,
        })
    }

    pub fn disassemble(&self) -> Result<Vec<Instruction>, AnalyzerError> {
        let disassembly = Disassembler::disassemble(self.cartridge.get_bytes())?;

        Ok(disassembly)
    }
}
