use super::cartridge::Cartridge;
use super::cartridge::{convert_ram_size, convert_rom_size, get_ram_size, get_rom_size};
use super::{map, Address, MemoryRange};

pub struct Mbc3Cartridge {
    rom_banks: Vec<[u8; 0x4000]>,
    ram_banks: Vec<[u8; 0x2000]>,
    ext_ram_enabled: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,
}

const ROM_BANK_SIZE: usize = 0x4000;
const RAM_BANK_SIZE: usize = 0x2000;
const RAM_ENABLE: MemoryRange = 0x0000..0x2000;
const ROM_BANK_NUMBER: MemoryRange = 0x2000..0x4000;
const RAM_BANK_NUMBER: MemoryRange = 0x4000..0x6000;
//const LATCH_CLOCK_DATA: MemoryRange = 0x6000..0x8000;

impl Mbc3Cartridge {
    pub fn new(cartridge: &[u8]) -> Mbc3Cartridge {
        let rom_size = convert_rom_size(get_rom_size(cartridge));
        let ram_size = convert_ram_size(get_ram_size(cartridge));

        assert!(
            rom_size * ROM_BANK_SIZE == cartridge.len(),
            "Cartridge data size should be equal to the reported number of rom banks"
        );

        let ram_banks = vec![[0; RAM_BANK_SIZE]; ram_size as usize];
        let rom_banks = (0..rom_size)
            .map(|i| {
                cartridge[ROM_BANK_SIZE * i..(ROM_BANK_SIZE * (i + 1))]
                    .try_into()
                    .unwrap()
            })
            .collect();

        Mbc3Cartridge {
            rom_banks,
            ram_banks,
            ext_ram_enabled: false,
            rom_bank_number: 0,
            ram_bank_number: 0,
        }
    }
}

impl Cartridge for Mbc3Cartridge {
    fn read(&self, address: Address) -> u8 {
        if map::EXT_WRAM.contains(&address) && !self.ext_ram_enabled {
            0 // TODO: check that disabled ram reads 0
        } else if map::EXT_WRAM.contains(&address) && self.ext_ram_enabled {
            self.ram_banks[self.ram_bank_number][address - map::EXT_WRAM.start]
        } else if map::ROM_BANK0.contains(&address) {
            self.rom_banks[0][address - map::ROM_BANK0.start]
        } else if map::ROM_BANK1.contains(&address) {
            self.rom_banks[self.rom_bank_number][address - map::ROM_BANK1.start]
        } else {
            panic!()
        }
    }
    fn write(&mut self, address: Address, value: u8) {
        if RAM_ENABLE.contains(&address) {
            self.ext_ram_enabled = value & 0xF == 0xA;
        } else if ROM_BANK_NUMBER.contains(&address) {
            // TODO: rom bank number upper bits
            let rom_bank_number = value & 0x1F;
            if rom_bank_number == 0 {
                self.rom_bank_number = 1;
            } else {
                self.rom_bank_number = rom_bank_number as usize; // TODO: needs to be masked to log2(#banks)
            }
        } else if RAM_BANK_NUMBER.contains(&address) {
            let ram_bank_number = value & 0x3;
            self.ram_bank_number = ram_bank_number as usize; // TODO: needs to be checked for number of ram
                                                             // banks
        } else if map::EXT_WRAM.contains(&address) && self.ext_ram_enabled {
            self.ram_banks[self.ram_bank_number][address - map::EXT_WRAM.start] = value;
        }
    }
}
