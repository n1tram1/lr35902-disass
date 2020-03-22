mod cartridge;
mod instruction;
mod disassembler;
mod error;

use cartridge::Cartridge;
use instruction::Instruction;
use disassembler::Disassembler;
use error::AnalyzerError;

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
