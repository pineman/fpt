use crate::memory::Address;

pub trait MemoryController {
    fn write(&self, address: Address, value: u8);
    fn read(&self, address: Address) -> u8;
}
