use crate::bus::Bus;
use crate::instruction::{AddrMode, Instruction, Mnemonic, Operand};
use crate::status::StatusFlags;

#[derive(Debug)]
enum Register {
    A,
    X,
    Y,
}

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

    pub fn reset(&mut self, bus: &mut impl Bus) {
        let lo = bus.read(0xFFFC);
        let hi = bus.read(0xFFFD);

        self.pc = u16::from_le_bytes([lo, hi]);
    }

    pub fn run(&mut self, bus: &mut impl Bus) {
        loop {
            self.step(bus)
        }
    }

    fn fetch(&mut self, bus: &mut impl Bus) -> u8 {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn step(&mut self, bus: &mut impl Bus) {
        let byte = self.fetch(bus);
        let instr = match Instruction::try_from(byte) {
            Ok(instr) => instr,
            Err(byte) => panic!(
                "unknown opcode {:#04X} pc: {:#X}",
                byte,
                self.pc.wrapping_sub(1)
            ),
        };

        let operand = self.resolve(bus, instr.mode);

        match instr.op {
            Mnemonic::Lda => self.load(bus, Register::A, operand),
            Mnemonic::Ldx => self.load(bus, Register::X, operand),
            Mnemonic::Ldy => self.load(bus, Register::Y, operand),
            Mnemonic::Sta => self.store(bus, Register::A, operand),
            Mnemonic::Stx => self.store(bus, Register::X, operand),
            Mnemonic::Sty => self.store(bus, Register::Y, operand),
            Mnemonic::Nop => {}
        }
    }

    fn resolve(&mut self, bus: &mut impl Bus, addr_mode: AddrMode) -> Operand {
        match addr_mode {
            AddrMode::Implied => Operand::None,
            AddrMode::Immediate => {
                let data = self.fetch(bus);

                Operand::Value(data)
            }
            AddrMode::Absolute => {
                let lo = self.fetch(bus);
                let hi = self.fetch(bus);

                Operand::Address(u16::from_le_bytes([lo, hi]))
            }
            AddrMode::ZeroPage => {
                let lo = self.fetch(bus);
                // ZeroPage from 0x0000 to 0x00FF
                Operand::Address(u16::from(lo))
            }
        }
    }

    fn load(&mut self, bus: &mut impl Bus, reg: Register, operand: Operand) {
        let value = operand.value(bus);
        *self.reg_mut(reg) = value;
        self.p.update_zn(value);
    }

    fn store(&self, bus: &mut impl Bus, reg: Register, operand: Operand) {
        let addr = operand.address();
        bus.write(addr, self.reg(reg));
    }

    fn reg_mut(&mut self, r: Register) -> &mut u8 {
        match r {
            Register::A => &mut self.a,
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
        }
    }

    fn reg(&self, r: Register) -> u8 {
        match r {
            Register::A => self.a,
            Register::X => self.x,
            Register::Y => self.y,
        }
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
            let mut mem = vec![0; 0x10000];
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

    fn setup(program: &[u8]) -> (Cpu, TestBus) {
        let bus = TestBus::with_program(0x0200, program);
        let mut cpu = Cpu::new();
        cpu.pc = 0x0200;
        (cpu, bus)
    }

    #[test]
    #[should_panic(expected = "unknown opcode")]
    fn unknown_opcode_panic() {
        let (mut cpu, mut bus) = setup(&[0x02]);
        cpu.step(&mut bus);
    }

    #[test]
    fn nop_advances_pc_by_one() {
        let (mut cpu, mut bus) = setup(&[0xEA]); // NOP
        cpu.step(&mut bus);
        assert_eq!(cpu.pc, 0x0201);
    }

    #[test]
    fn loads_into_correct_register() {
        // LDx #imm
        let cases = [
            (0xA9, Register::A),
            (0xA2, Register::X),
            (0xA0, Register::Y),
        ];

        for (opcode, reg) in cases {
            let (mut cpu, mut bus) = setup(&[opcode, 0x30]); // LDx #$30
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(reg), 0x30, "opcode {opcode:#04X}");
        }
    }

    #[test]
    fn store_from_correct_register() {
        // STx $abs
        let cases = [
            (0x8D, Register::A),
            (0x8E, Register::X),
            (0x8C, Register::Y),
        ];

        for (opcode, reg) in cases {
            let (mut cpu, mut bus) = setup(&[opcode, 0x34, 0x12]); // STx $1234
            *cpu.reg_mut(reg) = 0x30;
            cpu.step(&mut bus);
            assert_eq!(bus.mem[0x1234], 0x30, "opcode {opcode:#04X}");
        }
    }

    #[test]
    fn lda_immediate_loads_a() {
        let (mut cpu, mut bus) = setup(&[0xA9, 0x10]); // LDA #$10
        cpu.step(&mut bus);
        assert_eq!(cpu.pc, 0x0202);
        assert_eq!(cpu.a, 0x10);
    }

    #[test]
    fn lda_absolute_loads_a() {
        let (mut cpu, mut bus) = setup(&[0xAD, 0x34, 0x12]); // LDA $1234
        bus.mem[0x1234] = 0x05;
        cpu.step(&mut bus);
        assert_eq!(cpu.pc, 0x0203);
        assert_eq!(cpu.a, 0x05);
    }

    #[test]
    fn lda_zero_page_loads_a() {
        let (mut cpu, mut bus) = setup(&[0xA5, 0x30]); // LDA $30
        bus.mem[0x0030] = 0x05;
        cpu.step(&mut bus);
        assert_eq!(cpu.pc, 0x0202);
        assert_eq!(cpu.a, 0x05);
    }

    #[test]
    fn lda_check_zn_flags() {
        // val, Zero flag, Negative flag
        let cases = [
            (0b0000_0000, true, false),
            (0b0000_0001, false, false),
            (0b0100_0000, false, false),
            (0b1000_0000, false, true),
            (0b1111_1111, false, true),
        ];

        for (val, zero, negative) in cases {
            let (mut cpu, mut bus) = setup(&[0xA9, val]); // LDA #%val
            cpu.p.set(StatusFlags::ZERO, !zero);
            cpu.p.set(StatusFlags::NEGATIVE, !negative);
            cpu.step(&mut bus);
            assert_eq!(
                cpu.p.contains(StatusFlags::ZERO),
                zero,
                "value {val:#010b} == 0"
            );
            assert_eq!(
                cpu.p.contains(StatusFlags::NEGATIVE),
                negative,
                "value {val:#010b} < 0"
            );
        }
    }
}
