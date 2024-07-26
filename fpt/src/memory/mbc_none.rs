use super::cartridge::Cartridge;
use super::{map, Address};

/// Cartridge with no banking and no external ram
///
/// <https://gbdev.io/pandocs/nombc.html>
pub struct NoMbcCartridge {
    memory: [u8; 0x8000],
}

impl NoMbcCartridge {
    pub fn new(cartridge: &[u8]) -> NoMbcCartridge {
        assert!(
            cartridge.len() == 0x8000,
            "Expected cartridge size of 0x8000"
        );
        NoMbcCartridge {
            memory: cartridge.try_into().unwrap(),
        }
    }
}

impl Cartridge for NoMbcCartridge {
    fn read(&self, address: Address) -> u8 {
        assert!(
            map::ROM_BANK0.contains(&address) || map::ROM_BANK1.contains(&address),
            "Cartridge must only be accessed for cartridge specific memory segments"
        );
        self.memory[address]
    }

    fn write(&mut self, address: Address, _value: u8) {
        assert!(
            map::ROM_BANK0.contains(&address) || map::ROM_BANK1.contains(&address),
            "Cartridge must only be accessed for cartridge specific memory segments"
        );
    }
}
