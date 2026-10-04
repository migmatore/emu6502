use crate::bus::Bus;
use crate::instruction::{AddrMode, Instruction, Mnemonic};
use crate::status::StatusFlags;

#[derive(Debug, Default)]
pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: StatusFlags,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset<B: Bus>(&mut self, bus: &mut B) {
        let lo = bus.read(0xFFFC);
        let hi = bus.read(0xFFFD);

        self.pc = u16::from_le_bytes([lo, hi]);
    }

    pub fn run<B: Bus>(&mut self, bus: &mut B) {
        loop {
            self.step(bus)
        }
    }

    fn step<B: Bus>(&mut self, bus: &mut B) {
        let byte = self.fetch(bus);
        let instr = match Instruction::try_from(byte) {
            Ok(instr) => instr,
            Err(byte) => panic!("unknown opcode {:#04X}", byte),
        };

        match (instr.op, instr.mode) {
            (Mnemonic::Lda, AddrMode::Immediate) => {
                let data = self.fetch(bus);
                self.a = data;
            }
            _ => {}
        }
    }

    fn fetch<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
}
