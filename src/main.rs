use crate::bus::Bus;
use crate::cpu::Cpu;

mod bus;
mod cpu;
mod instruction;
mod status;

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
            0x8000..=u16::MAX => self.rom[(addr - 0x8000) as usize],
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..0x8000 => self.ram[addr as usize] = data,
            0x8000..=u16::MAX => self.rom[(addr - 0x8000) as usize] = data,
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
