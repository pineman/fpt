use crate::memory::GBAddress;
use crate::memory::memory::MemoryRange;

pub trait MemoryController {
    fn write(&mut self, address: GBAddress, value: u8, cartridge: &mut Vec<u8>);
    fn read(&self, address: GBAddress, cartridge: &Vec<u8>) -> u8;
    fn slice(&self, range: MemoryRange, cartridge: &Vec<u8>) -> &[u8];
    fn slice_mut(&mut self, range: MemoryRange, cartridge: &Vec<u8>) -> &mut [u8];
}
