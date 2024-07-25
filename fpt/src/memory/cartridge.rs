use crate::memory::map;
use crate::memory::{Address, MemoryRange};

pub fn get_cartridge_type(tape: &[u8]) -> u8 {
    tape[map::CARTRIDGE_TYPE]
}

pub trait Cartridge {
    fn read(&self, address: Address) -> u8;
    fn write(&mut self, address: Address, value: u8);

    fn read_range(&self, memory_range: MemoryRange) -> Vec<u8> {
        memory_range.map(|address| self.read(address)).collect()
    }

    fn get_title(&self) -> String {
        String::from_utf8(self.read_range(map::TITLE)).unwrap()
    }

    fn get_manufacturer_code(&self) -> String {
        String::from_utf8(self.read_range(map::MANUFACTURER_CODE)).unwrap()
    }

    fn get_new_licensee_code(&self) -> String {
        String::from_utf8(self.read_range(map::NEW_LICENSEE_CODE)).unwrap()
    }

    fn get_sgb_flag(&self) -> u8 {
        self.read(map::SGB_FLAG)
    }

    fn get_cartridge_type(&self) -> u8 {
        self.read(map::CARTRIDGE_TYPE)
    }

    fn get_rom_size(&self) -> u8 {
        self.read(map::ROM_SIZE)
    }

    fn get_ram_size(&self) -> u8 {
        self.read(map::RAM_SIZE)
    }

    fn get_old_licensee_code(&self) -> u8 {
        self.read(map::OLD_LICENSEE_CODE)
    }

    fn get_version_number(&self) -> u8 {
        self.read(map::VERSION_NUMBER)
    }
}

pub struct EmptyCartridge {}

impl EmptyCartridge {
    pub fn new() -> EmptyCartridge {
        EmptyCartridge {}
    }
}

impl Cartridge for EmptyCartridge {
    fn read(&self, _address: Address) -> u8 {
        0xFF
    }
    fn write(&mut self, _address: Address, _value: u8) {}
}
