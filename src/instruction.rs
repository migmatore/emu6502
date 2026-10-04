#[derive(Debug)]
pub enum Mnemonic {
    Lda,
    Sta,
    Nop,
}

#[derive(Debug)]
pub enum AddrMode {
    Implied,
    Immediate,
    Absolute,
}

#[derive(Debug)]
pub struct Instruction {
    pub op: Mnemonic,
    pub mode: AddrMode,
}

impl TryFrom<u8> for Instruction {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Instruction, u8> {
        let (op, mode) = match byte {
            0xA9 => (Mnemonic::Lda, AddrMode::Immediate),
            0x8D => (Mnemonic::Sta, AddrMode::Absolute),
            0xEA => (Mnemonic::Nop, AddrMode::Implied),
            _ => return Err(byte),
        };

        Ok(Instruction { op, mode })
    }
}
