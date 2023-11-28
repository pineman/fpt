/// Memory controller 0. No memory controller
use crate::memory::GBAddress;
use crate::memory::MemoryController;

pub struct Mbc0 {}

impl Mbc0 {
    pub fn new() -> Mbc0 {
        Mbc0 {}
    }
}

impl MemoryController for Mbc0 {
    fn write(&mut self, address: GBAddress, value: u8, cartridge: &mut Vec<u8>) {
        cartridge[address as usize] = value
    }

    fn read(&self, address: GBAddress, cartridge: &Vec<u8>) -> u8 {
        dbg!(cartridge);
        cartridge[address as usize]
    }
}
