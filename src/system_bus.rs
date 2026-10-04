use crate::bus::Bus;

pub struct SystemBus {
    ram: Box<[u8; 32768]>,
    rom: Box<[u8; 32768]>,
}

impl SystemBus {
    pub fn new() -> Self {
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
