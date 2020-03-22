pub struct Instruction {
    opcode: u8,
}

impl Instruction {
    pub fn new(opcode: u8) -> Instruction {
        Instruction { opcode }
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.opcode)
    }
}
