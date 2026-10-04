use crate::status::StatusFlags;

mod status;

trait Bus {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

enum Mnemonic {
    Lda,
    Sta,
    Nop,
}

enum AddrMode {
    Implied,
    Immediate,
    Absolute,
}

struct Instruction {
    op: Mnemonic,
    mode: AddrMode,
}

#[derive(Debug, Default)]
struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: StatusFlags,
}

impl Cpu {
    fn new() -> Self {
        Self::default()
    }

    fn reset<B: Bus>(&mut self, bus: &mut B) {
        let lo = bus.read(0xFFFC);
        let hi = bus.read(0xFFFD);

        self.pc = u16::from_be_bytes([hi, lo]);
    }

    fn run<B: Bus>(&mut self, bus: &mut B) {
        loop {
            self.step(bus)
        }
    }

    fn step<B: Bus>(&mut self, bus: &mut B) {
        let byte = self.fetch(bus);
        let instr = match Self::decode(byte) {
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

    fn decode(byte: u8) -> Result<Instruction, u8> {
        let (op, mode) = match byte {
            0xA9 => (Mnemonic::Lda, AddrMode::Immediate),
            0x8D => (Mnemonic::Sta, AddrMode::Absolute),
            0xEA => (Mnemonic::Nop, AddrMode::Implied),
            _ => return Err(byte),
        };

        Ok(Instruction { op, mode })
    }
}

struct SystemBus {
    ram: Box<[u8; 32768]>,
    rom: Box<[u8; 32768]>,
}

impl SystemBus {
    fn new() -> Self {
        Self {
            ram: Box::new([0; 32768]),
            rom: Box::new([0; 32768]),
        }
    }
}

impl Bus for SystemBus {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..0x8000 => self.ram[addr as usize],
            0x8000..=0xFFFF => self.rom[(addr - 0x8000) as usize],
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..0x8000 => self.ram[addr as usize] = data,
            0x8000..=0xFFFF => self.rom[(addr - 0x8000) as usize] = data,
            _ => {}
        }
    }
}

struct MyBoard {
    cpu: Cpu,
    bus: SystemBus,
}

impl MyBoard {
    fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            bus: SystemBus::new(),
        }
    }

    fn init(&mut self) {
        self.bus.write(0xFFFC, 0x00);
        self.bus.write(0xFFFD, 0x80);
        self.bus.write(0x8000, 0xA9);
        self.bus.write(0x8001, 0x05);

        self.cpu.reset(&mut self.bus);
    }

    fn start(&mut self) {
        self.cpu.run(&mut self.bus)
    }
}

fn main() {
    let mut board = MyBoard::new();
    board.init();
    board.start();

    println!("Hello, world!");
}
