use crate::memory::GBAddress;

pub trait MemoryController {
    fn write(&mut self, address: GBAddress, value: u8, cartridge: &mut Vec<u8>);
    fn read(&self, address: GBAddress, cartridge: &Vec<u8>) -> u8;
}
