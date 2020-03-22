pub enum AnalyzerError {
    InvalidOpcode(u8),
    InvalidInstructionSize(usize),
    InvalidCartridge(std::io::Error),
}

impl std::fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidOpcode(op) => write!(f, "opcode {:X} is invalid", op),
            Self::InvalidInstructionSize(size) => write!(f, "invalid instruction size: {}", size),
            Self::InvalidCartridge(ref e) => {
                write!(f, "invalid cartridge, got: ")?;
                e.fmt(f)
            },
        }
    }
}

impl std::fmt::Debug for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[file: {}, line: {}] {}", file!(), line!(), self)
    }
}

impl std::error::Error for AnalyzerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidOpcode(_) => None,
            Self::InvalidInstructionSize(_) => None,
            Self::InvalidCartridge(ref e) => Some(e),
        }
    }
}

impl From<std::io::Error> for AnalyzerError {
    fn from(err: std::io::Error) -> AnalyzerError {
        AnalyzerError::InvalidCartridge(err)
    }
}
