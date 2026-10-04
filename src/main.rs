use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::system_bus::SystemBus;

mod bus;
mod cpu;
mod instruction;
mod status;
mod system_bus;

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
        self.bus.write(0xFFFC, 0x00); // reset vector low byte
        self.bus.write(0xFFFD, 0x80); // reset vector high byte
        self.bus.write(0x8000, 0xA9); // LDA #$05
        self.bus.write(0x8001, 0x05); //
        self.bus.write(0x8002, 0x8D); // STA $6020
        self.bus.write(0x8003, 0x20); //
        self.bus.write(0x8004, 0x60); //
        self.bus.write(0x8005, 0xEA); // NOP

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
}
