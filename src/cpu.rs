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
            Err(byte) => panic!(
                "unknown opcode {:#04X} pc: {:#X}",
                byte,
                self.pc.wrapping_sub(1)
            ),
        };

        match (instr.op, instr.mode) {
            (Mnemonic::Lda, AddrMode::Immediate) => {
                let data = self.fetch(bus);
                self.a = data;
            }
            (Mnemonic::Sta, AddrMode::Absolute) => {
                let lo = self.fetch(bus);
                let hi = self.fetch(bus);

                bus.write(u16::from_le_bytes([lo, hi]), self.a);
            }
            (Mnemonic::Nop, AddrMode::Implied) => {}
            _ => todo!("unimplemented instruction {:?}", instr),
        }
    }

    fn fetch<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBus {
        mem: Vec<u8>,
    }

    impl TestBus {
        fn with_program(start: u16, program: &[u8]) -> Self {
            let mut mem = vec![0; 0xFF];
            let start = start as usize;

            mem[start..start + program.len()].copy_from_slice(program);

            Self { mem }
        }
    }

    impl Bus for TestBus {
        fn read(&mut self, addr: u16) -> u8 {
            self.mem[addr as usize]
        }

        fn write(&mut self, addr: u16, data: u8) {
            self.mem[addr as usize] = data;
        }
    }

    #[test]
    #[should_panic(expected = "unknown opcode")]
    fn unknown_opcode_panic() {
        let mut bus = TestBus::with_program(0x0, &[0x02]);
        let mut cpu = Cpu::new();
        cpu.pc = 0x0;
        cpu.step(&mut bus);
    }
}
