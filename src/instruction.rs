use crate::bus::Bus;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operand {
    None,
    Value(u8),
    Address(u16),
}

impl Operand {
    pub fn value(self, bus: &mut impl Bus) -> u8 {
        match self {
            Operand::None => unreachable!("instruction needs a value, got {:?}", self),
            Operand::Value(v) => v,
            Operand::Address(addr) => bus.read(addr),
        }
    }

    pub fn address(self) -> u16 {
        match self {
            Operand::Address(addr) => addr,
            _ => unreachable!("instruction needs an address, got {:?}", self),
        }
    }
}

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
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
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
            0xA5 => (Mnemonic::Lda, AddrMode::ZeroPage),
            0xB5 => (Mnemonic::Lda, AddrMode::ZeroPageX),
            0xAD => (Mnemonic::Lda, AddrMode::Absolute),
            0xBD => (Mnemonic::Lda, AddrMode::AbsoluteX),
            0xB9 => (Mnemonic::Lda, AddrMode::AbsoluteY),
            0xA2 => (Mnemonic::Ldx, AddrMode::Immediate),
            0xA6 => (Mnemonic::Ldx, AddrMode::ZeroPage),
            0xB6 => (Mnemonic::Ldx, AddrMode::ZeroPageY),
            0xAE => (Mnemonic::Ldx, AddrMode::Absolute),
            0xA0 => (Mnemonic::Ldy, AddrMode::Immediate),
            0xA4 => (Mnemonic::Ldy, AddrMode::ZeroPage),
            0xB4 => (Mnemonic::Ldy, AddrMode::ZeroPageX),
            0xAC => (Mnemonic::Ldy, AddrMode::Absolute),
            0x85 => (Mnemonic::Sta, AddrMode::ZeroPage),
            0x8D => (Mnemonic::Sta, AddrMode::Absolute),
            0x86 => (Mnemonic::Stx, AddrMode::ZeroPage),
            0x8E => (Mnemonic::Stx, AddrMode::Absolute),
            0x84 => (Mnemonic::Sty, AddrMode::ZeroPage),
            0x8C => (Mnemonic::Sty, AddrMode::Absolute),
            0xEA => (Mnemonic::Nop, AddrMode::Implied),
            _ => return Err(byte),
        };

        Ok(Instruction { op, mode })
    }
}
