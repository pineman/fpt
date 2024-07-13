use super::cartridge::Cartridge;
use super::{map, Address, MemoryRange};

pub struct NoMbcCartridge {
    memory: Vec<u8>,
}

impl NoMbcCartridge {
    pub fn new(cartridge: &[u8]) -> NoMbcCartridge {
        NoMbcCartridge {
            memory: cartridge.to_vec(),
        }
    }
}

impl Cartridge for NoMbcCartridge {
    fn read(&self, address: Address) -> u8 {
        self.memory[address]
    }
    fn write(&mut self, address: Address, value: u8) {
        if map::EXT_RAM.contains(&address) {
            self.memory[address] = value;
        }
    }

    fn read_range(&self, memory_range: MemoryRange) -> Vec<u8> {
        self.memory[memory_range].to_vec()
    }
}
