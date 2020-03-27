use super::error::AnalyzerError;

#[derive(Debug)]
enum Mnemonic {
    NOP,
    STOP,
    LD, /* Load */
    LDIL, /* Load lhs-increment */
    LDDL, /* Load lhs-decrement */
    LDIR, /* Load rhs-increment */
    LDDR, /* Load rhs-decrement */
    JR,
    JP,
    JRNZ,
    JRZ,
    JRNC,
    JRC,
    ADD,
    SUB,
    INC,
    DEC,
    AND,
    OR,
    PUSH,
    POP,
    CALL,
    RET,
    RLC,
    RRC,
    RL,
    RR,
    DA,
    CPL,
    SCF,
    CCF,
    HALT,
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

    SP,
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
    Imm8(u8),
    Imm16(u16),
    Addr8(u8),
    DerefAddr8(u8),
    Addr16(u16),
    DerefAddr16(u16),
    Rel8(u8),
    Reg(Register),
    DerefReg(Register),
}

impl Operand {
    pub fn size(&self) -> usize {
        match self {
            Self::Reg(r) | Self::DerefReg(r) => r.size(),
            Self::Imm16(_) | Self::Addr16(_) => 2,
            _ => 1,
        }
    }
}

pub struct Instruction {
    size: usize,
    cycles: usize,
    mnemonic: Mnemonic,
    lhs: Option<Operand>,
    rhs: Option<Operand>,
}

impl Instruction {
    pub fn from_slice(bytes: &[u8]) -> Result<Instruction, AnalyzerError> {
        if bytes.len() < 1 {
            return Err(AnalyzerError::InvalidInstructionSize(0));
        }

        Instruction::decode(bytes)
    }

    fn decode(bytes: &[u8]) -> Result<Instruction, AnalyzerError> {
        let opcode = bytes[0];

        let inst = match opcode {
            0x00 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::NOP,
                lhs: None,
                rhs: None,
            },
            0x01 => Instruction {
                size: 3,
                cycles: 12,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::BC)),
                rhs: Some(Operand::Imm16(Instruction::read_imm16(bytes)?)),
            },
            0x02 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefReg(Register::BC)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x03 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::BC)),
                rhs: None,
            },
            0x04 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: None,
            },
            0x05 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: None,
            },
            0x06 => Instruction {
                size: 2,
                cycles: 8,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: Some(Operand::Imm8(Instruction::read_imm8(bytes)?)),
            },
            0x07 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::RLC,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: None,
            },
            0x08 => Instruction {
                size: 3,
                cycles: 20,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefAddr16(Instruction::read_imm16(bytes)?)),
                rhs: Some(Operand::Reg(Register::SP)),
            },
            0x09 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::ADD,
                lhs: Some(Operand::Reg(Register::HL)),
                rhs: Some(Operand::Reg(Register::BC)),
            },
            0x0A => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::DerefReg(Register::BC)),
            },
            0x0B => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::BC)),
                rhs: None,
            },
            0x0C => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: None,
            },
            0x0D => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: None,
            },
            0x0E => Instruction {
                size: 2,
                cycles: 8,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: Some(Operand::Imm8(Instruction::read_imm8(bytes)?)),
            },
            0x0F => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::RRC,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: None,
            },
            0x10 => Instruction {
                size: 2,
                cycles: 4,
                mnemonic: Mnemonic::STOP,
                lhs: Some(Operand::Imm8(0)),
                rhs: None,
            },
            0x11 => Instruction {
                size: 3,
                cycles: 12,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::DE)),
                rhs: Some(Operand::Imm16(Instruction::read_imm16(bytes)?)),
            },
            0x12 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefReg(Register::DE)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x13 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::DE)),
                rhs: None,
            },
            0x14 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: None,
            },
            0x15 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: None,
            },
            0x16 => Instruction {
                size: 2,
                cycles: 8,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: Some(Operand::Imm8(Instruction::read_imm8(bytes)?)),
            },
            0x17 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::RL,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: None,
            },
            0x18 => Instruction {
                size: 2,
                cycles: 12,
                mnemonic: Mnemonic::JR,
                lhs: Some(Operand::Rel8(Instruction::read_imm8(bytes)?)),
                rhs: None,
            },
            0x19 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::ADD,
                lhs: Some(Operand::Reg(Register::HL)),
                rhs: Some(Operand::Reg(Register::DE)),
            },
            0x1A => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::DerefReg(Register::DE)),
            },
            0x1B => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::DE)),
                rhs: None,
            },
            0x1C => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: None,
            },
            0x1D => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: None,
            },
            0x1E => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::E)),
                rhs: Some(Operand::Imm8(Instruction::read_imm8(bytes)?)),
            },
            0x1F => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::RR,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: None,
            },
            0x20 => Instruction {
                size: 2,
                cycles: 0, /* Cycle depends on branch taken (12/8 true/false). */
                mnemonic: Mnemonic::JRNZ,
                lhs: Some(Operand::Rel8(Instruction::read_imm8(bytes)?)),
                rhs: None,
            },
            0x21 => Instruction {
                size: 3,
                cycles: 12,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::HL)),
                rhs: Some(Operand::Imm16(Instruction::read_imm16(bytes)?)),
            },
            0x22 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::LDIL,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x23 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::DE)),
                rhs: None,
            },
            0x24 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: None,
            },
            0x25 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: None,
            },
            0x26 => Instruction {
                size: 2,
                cycles: 6,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: Some(Operand::Imm8(Instruction::read_imm8(bytes)?)),
            },
            0x27 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::DA,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: None,
            },
            0x28 => Instruction {
                size: 2,
                cycles: 0, /* Cycle depends on branch taken (12/8 true/false). */
                mnemonic: Mnemonic::JRZ,
                lhs: Some(Operand::Rel8(Instruction::read_imm8(bytes)?)),
                rhs: None,
            },
            0x29 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::ADD,
                lhs: Some(Operand::Reg(Register::HL)),
                rhs: Some(Operand::Reg(Register::HL)),
            },
            0x2A => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::LDIR,
                lhs: Some(Operand::DerefReg(Register::A)),
                rhs: Some(Operand::Reg(Register::HL)),
            },
            0x2B => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::HL)),
                rhs: None,
            },
            0x2C => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: None,
            },
            0x2D => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: None,
            },
            0x2E => Instruction {
                size: 2,
                cycles: 8,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: Some(Operand::Imm8(Instruction::read_imm8(bytes)?)),
            },
            0x2F => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::CPL,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: None,
            },
            0x30 => Instruction {
                size: 2,
                cycles: 0, /* Cycle depends on branch taken (12/8 true/false). */
                mnemonic: Mnemonic::JRNC,
                lhs: Some(Operand::Rel8(Instruction::read_imm8(bytes)?)),
                rhs: None,
            },
            0x31 => Instruction {
                size: 3,
                cycles: 12,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::SP)),
                rhs: Some(Operand::Imm16(Instruction::read_imm16(bytes)?)),
            },
            0x32 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::LDDL,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x33 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::SP)),
                rhs: None,
            },
            0x34 => Instruction {
                size: 1,
                cycles: 12,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: None,
            },
            0x35 => Instruction {
                size: 1,
                cycles: 12,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: None,
            },
            0x36 => Instruction {
                size: 2,
                cycles: 12,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: Some(Operand::Imm8(Instruction::read_imm8(bytes)?)),
            },
            0x37 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::SCF,
                lhs: None,
                rhs: None,
            },
            0x38 => Instruction {
                size: 2,
                cycles: 0, /* Cycle depends on branch taken (12/8 true/false). */
                mnemonic: Mnemonic::JRC,
                lhs: Some(Operand::Imm8(Instruction::read_imm8(bytes)?)),
                rhs: None,
            },
            0x39 => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::ADD,
                lhs: Some(Operand::Reg(Register::HL)),
                rhs: Some(Operand::Reg(Register::SP)),
            },
            0x3A => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::LDDR,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::DerefReg(Register::HL)),
            },
            0x3B => Instruction {
                size: 1,
                cycles: 8,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::SP)),
                rhs: None,
            },
            0x3C => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::INC,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: None,
            },
            0x3D => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::DEC,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: None,
            },
            0x3E => Instruction {
                size: 2,
                cycles: 8,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::Imm8(Instruction::read_imm8(bytes)?)),
            },
            0x3F => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::CCF,
                lhs: None,
                rhs: None,
            },
            0x40 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: Some(Operand::Reg(Register::B)),
            },
            0x41 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: Some(Operand::Reg(Register::C)),
            },
            0x42 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: Some(Operand::Reg(Register::D)),
            },
            0x43 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: Some(Operand::Reg(Register::E)),
            },
            0x44 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: Some(Operand::Reg(Register::H)),
            },
            0x45 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: Some(Operand::Reg(Register::L)),
            },
            0x46 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: Some(Operand::DerefReg(Register::HL)),
            },
            0x47 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::B)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x48 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: Some(Operand::Reg(Register::B)),
            },
            0x49 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: Some(Operand::Reg(Register::C)),
            },
            0x4A => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: Some(Operand::Reg(Register::D)),
            },
            0x4B => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: Some(Operand::Reg(Register::E)),
            },
            0x4C => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: Some(Operand::Reg(Register::H)),
            },
            0x4D => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: Some(Operand::Reg(Register::L)),
            },
            0x4E => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: Some(Operand::DerefReg(Register::HL)),
            },
            0x4F => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::C)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x50 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: Some(Operand::Reg(Register::B)),
            },
            0x51 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: Some(Operand::Reg(Register::C)),
            },
            0x52 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: Some(Operand::Reg(Register::D)),
            },
            0x53 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: Some(Operand::Reg(Register::E)),
            },
            0x54 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: Some(Operand::Reg(Register::H)),
            },
            0x55 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: Some(Operand::Reg(Register::L)),
            },
            0x56 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: Some(Operand::DerefReg(Register::HL)),
            },
            0x57 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::D)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x58 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::E)),
                rhs: Some(Operand::Reg(Register::B)),
            },
            0x58 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::E)),
                rhs: Some(Operand::Reg(Register::C)),
            },
            0x59 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::E)),
                rhs: Some(Operand::Reg(Register::C)),
            },
            0x5A => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::E)),
                rhs: Some(Operand::Reg(Register::D)),
            },
            0x5B => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::E)),
                rhs: Some(Operand::Reg(Register::E)),
            },
            0x5C => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::E)),
                rhs: Some(Operand::Reg(Register::H)),
            },
            0x5E => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::E)),
                rhs: Some(Operand::DerefReg(Register::HL)),
            },
            0x5F => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::E)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x60 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: Some(Operand::Reg(Register::B)),
            },
            0x61 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: Some(Operand::Reg(Register::C)),
            },
            0x62 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: Some(Operand::Reg(Register::D)),
            },
            0x63 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: Some(Operand::Reg(Register::E)),
            },
            0x64 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: Some(Operand::Reg(Register::H)),
            },
            0x65 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: Some(Operand::Reg(Register::L)),
            },
            0x66 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: Some(Operand::DerefReg(Register::HL)),
            },
            0x67 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::H)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x68 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: Some(Operand::Reg(Register::B)),
            },
            0x69 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: Some(Operand::Reg(Register::C)),
            },
            0x6A => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: Some(Operand::Reg(Register::D)),
            },
            0x6B => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: Some(Operand::Reg(Register::E)),
            },
            0x6C => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: Some(Operand::Reg(Register::H)),
            },
            0x6D => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: Some(Operand::Reg(Register::L)),
            },
            0x6E => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: Some(Operand::DerefReg(Register::HL)),
            },
            0x6F => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::L)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x70 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: Some(Operand::Reg(Register::B)),
            },
            0x71 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: Some(Operand::Reg(Register::C)),
            },
            0x72 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: Some(Operand::Reg(Register::D)),
            },
            0x73 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: Some(Operand::Reg(Register::E)),
            },
            0x74 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: Some(Operand::Reg(Register::H)),
            },
            0x75 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: Some(Operand::Reg(Register::L)),
            },
            0x76 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::HALT,
                lhs: None,
                rhs: None,
            },
            0x77 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::DerefReg(Register::HL)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            0x78 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::Reg(Register::B)),
            },
            0x79 => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::Reg(Register::C)),
            },
            0x7A => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::Reg(Register::D)),
            },
            0x7B => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::Reg(Register::E)),
            },
            0x7C => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::Reg(Register::E)),
            },
            0x7D => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::Reg(Register::L)),
            },
            0x7E => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::DerefReg(Register::HL)),
            },
            0x7F => Instruction {
                size: 1,
                cycles: 4,
                mnemonic: Mnemonic::LD,
                lhs: Some(Operand::Reg(Register::A)),
                rhs: Some(Operand::Reg(Register::A)),
            },
            _ => return Err(AnalyzerError::InvalidOpcode(opcode)),
        };

        Ok(inst)
    }

    fn read_imm8(bytes: &[u8]) -> Result<u8, AnalyzerError> {
        /* bytes[0] is the opcodes, operands are after */
        if bytes.len() < 2 {
            return Err(AnalyzerError::InvalidInstructionSize(bytes.len()));
        }

        Ok(u8::from_le_bytes([bytes[1]]))
    }

    fn read_imm16(bytes: &[u8]) -> Result<u16, AnalyzerError> {
        /* bytes[0] is the opcodes, operands are after */
        if bytes.len() < 3 {
            return Err(AnalyzerError::InvalidInstructionSize(bytes.len()));
        }

        Ok(u16::from_le_bytes([bytes[1], bytes[2]]))
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
