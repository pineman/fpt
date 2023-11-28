use crate::memory::GBAddress;
use crate::memory::MemoryController;

pub struct Mbc1 {}

impl Mbc1 {
    pub fn new() -> Mbc1 {
        Mbc1 {}
    }
}

impl MemoryController for Mbc1 {
    fn write(&mut self, address: GBAddress, value: u8, cartridge: &mut Vec<u8>) {
        if 0x0000 <= address && address <= 0x1fff {
            if value & 0xF == 0xA {
                // ram enable
            } else {
                // ram disable
            }
        } else if 0x2000 <= address && address <= 0x3fff {
            let rom_select = value & 0x1F;
        } else if 0x4000 <= address && address <= 0x5fff {
        }
    }

    fn read(&self, _address: GBAddress, cartridge: &Vec<u8>) -> u8 {
        0
    }
}
