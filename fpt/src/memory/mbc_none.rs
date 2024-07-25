use super::cartridge::Cartridge;
use super::{Address, MemoryRange};

pub struct NoMbcCartridge {
    memory: Vec<u8>,
}

impl NoMbcCartridge {
    pub fn new(cartridge: &[u8]) -> NoMbcCartridge {
        let mut cartridge = cartridge.to_vec();
        let mut padding = vec![0; (0x10000 - cartridge.len()).max(0)];
        cartridge.append(&mut padding);
        NoMbcCartridge { memory: cartridge }
    }
}

impl Cartridge for NoMbcCartridge {
    fn read(&self, address: Address) -> u8 {
        self.memory[address]
    }
    fn write(&mut self, address: Address, value: u8) {
        self.memory[address] = value;
    }

    fn read_range(&self, memory_range: MemoryRange) -> Vec<u8> {
        self.memory[memory_range].to_vec()
    }
}
