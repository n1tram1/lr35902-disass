use super::error::AnalyzerError;
use super::instruction::Instruction;

pub struct Disassembler;

impl Disassembler {
    pub fn disassemble(bytes: &Vec<u8>) -> Result<Vec<Instruction>, AnalyzerError> {
        let mut instructions: Vec<Instruction> = Vec::new();

        let mut i = 0;
        while i < bytes.len() {
            let inst = Instruction::from_slice(&bytes[i..])?;
            i += inst.size();

            instructions.push(inst);
        }

        Ok(instructions)
    }
}
