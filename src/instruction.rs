#[derive(Debug, Copy, Clone)]
pub enum Mnemonic {
    Lda,
    Ldx,
    Ldy,
    Sta,
    Stx,
    Sty,
    Nop,
}

#[derive(Debug, Copy, Clone)]
pub enum AddrMode {
    Implied,
    Immediate,
    Absolute,
    ZeroPage,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub op: Mnemonic,
    pub mode: AddrMode,
}

impl TryFrom<u8> for Instruction {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Instruction, Self::Error> {
        let (op, mode) = match byte {
            0xA9 => (Mnemonic::Lda, AddrMode::Immediate),
            0xA2 => (Mnemonic::Ldx, AddrMode::Immediate),
            0xA0 => (Mnemonic::Ldy, AddrMode::Immediate),
            0x8D => (Mnemonic::Sta, AddrMode::Absolute),
            0x8E => (Mnemonic::Stx, AddrMode::Absolute),
            0x8C => (Mnemonic::Sty, AddrMode::Absolute),
            0xEA => (Mnemonic::Nop, AddrMode::Implied),
            _ => return Err(byte),
        };

        Ok(Instruction { op, mode })
    }
}
