use super::error::AnalyzerError;

#[derive(Debug)]
enum Mnemonic {
    NOP,
    STOP,
    LD,
    JR,
    ADD,
    SUB,
    AND,
    OR,
    PUSH,
    POP,
    CALL,
    RET,
}

#[derive(Debug)]
enum Register {
    AF,
    A,
    F,

    BC,
    B,
    C,

    DE,
    D,
    E,

    HL,
    H,
    L,
}

impl Register {
    pub fn size(&self) -> usize {
        match *self {
            Self::AF | Self::BC | Self::DE | Self::HL => 2,
            _ => 1,
        }
    }
}

#[derive(Debug)]
enum Operand {
    Data8(u8),
    Data16(u16),
    Addr8(u8),
    Addr16(u16),
    Rel8(u8),
    Reg(Register),
    DerefReg(Register),
}

impl Operand {
    pub fn size(&self) -> usize {
        match self {
            Self::Reg(r) | Self::DerefReg(r) => r.size(),
            Self::Data16(_) | Self::Addr16(_) => 2,
            _ => 1,
        }
    }
}

pub struct Instruction {
    opcode: u8,
    mnemonic: Mnemonic,
    lhs: Option<Operand>,
    rhs: Option<Operand>,
}

impl Instruction {
    fn new(
        opcode: u8,
        mnemonic: Mnemonic,
        lhs: Option<Operand>,
        rhs: Option<Operand>,
    ) -> Instruction {
        Instruction {
            opcode,
            mnemonic,
            lhs,
            rhs,
        }
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Instruction, AnalyzerError> {
        if bytes.len() < 1 {
            return Err(AnalyzerError::InvalidInstructionSize(0));
        }

        // let inst = decode(bytes);
        let opcode = bytes[0];
        let inst = match opcode {
            0x00 => Instruction::new(opcode, Mnemonic::NOP, None, None),
            0x01 => Instruction::new(
                opcode,
                Mnemonic::LD,
                Some(Operand::Reg(Register::BC)),
                Some(Operand::Data16(u16::from_le_bytes([bytes[1], bytes[2]]))),
            ),
            // 0x02 => instruction!(opcode, Mnemonic::LD, Operand::DerefReg(Register::BC), Operand::Reg(Register::A)),
            0x10 => Instruction::new(opcode, Mnemonic::STOP, Some(Operand::Data8(0)), None),
            _ => return Err(AnalyzerError::InvalidOpcode(opcode)),
        };

        Ok(inst)
    }

    pub fn size(&self) -> usize {
        let mut len = 1;

        if let Some(lhs) = &self.lhs {
            len += lhs.size();
        }

        if let Some(rhs) = &self.rhs {
            len += rhs.size();
        }

        len
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(f, "{:?}", self.mnemonic);

        if let Some(lhs) = &self.lhs {
            write!(f, " {:?}", lhs)?;
        }

        if let Some(rhs) = &self.rhs {
            write!(f, " {:?}", rhs)?;
        }

        res
    }
}
