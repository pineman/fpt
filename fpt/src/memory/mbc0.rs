/// Memory controller 0. No memory controller

use crate::memory::GBAddress;
use crate::memory::MemoryController;
use crate::memory::memory::MemoryRange;

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

    fn slice(&self, range: MemoryRange, cartridge: &Vec<u8>) -> &[u8] {
        &[]
    }

    fn slice_mut(&mut self, range: MemoryRange, cartridge: &Vec<u8>) -> &mut [u8] {
        &mut []
    }
}
