use super::instruction::Instruction;

pub struct Disassembler {
}

impl Disassembler {
    pub fn disassemble(bytes: &Vec<u8>) -> Vec<Instruction> {
        // TODO: implement this
        bytes.iter().map(|&b| Instruction::new(b)).collect()
    }
}
