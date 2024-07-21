use super::cartridge::{convert_ram_size, convert_rom_size, get_ram_size, get_rom_size};
use super::{map, Address, Cartridge, MemoryRange};

pub struct Mbc3Cartridge {
    memory: Vec<u8>,
    rom_banks: Vec<[u8; 0x4000]>,
    ram_banks: Vec<[u8; 0x2000]>,
    ext_ram_enabled: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,
}

impl Mbc3Cartridge {
    pub fn new(cartridge: &[u8]) -> Mbc3Cartridge {
        let rom_size = convert_rom_size(get_rom_size(cartridge));
        let ram_size = convert_ram_size(get_ram_size(cartridge));
        let mut rom_banks = vec![[0; 0x4000]; rom_size as usize];
        let mut ram_banks = vec![[0; 0x2000]; ram_size as usize];

        // TODO: wtf is this initialization
        for i in 0..rom_size {
            for j in 0..0x4000 {
                rom_banks[i][j] = cartridge[0x4000 * i + j];
            }
        }

        for i in 0..ram_size {
            for j in 0..0x2000 {
                ram_banks[i][j] = cartridge[0x2000 * i + j];
            }
        }

        Mbc3Cartridge {
            memory: cartridge.to_vec(),
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
        if map::EXT_RAM.contains(&address) && !self.ext_ram_enabled {
            0 // TODO: check that disabled ram reads 0
        } else if map::EXT_RAM.contains(&address) && self.ext_ram_enabled {
            self.ram_banks[self.ram_bank_number][address - map::EXT_RAM.start]
        } else if map::ROM_BANK0.contains(&address) {
            self.rom_banks[0][address - map::ROM_BANK0.start]
        } else if map::ROM_BANK1.contains(&address) {
            self.rom_banks[self.rom_bank_number][address - map::ROM_BANK1.start]
        } else {
            self.memory[address]
        }
    }
    fn write(&mut self, address: Address, value: u8) {
        if (0x0000..0x2000).contains(&address) {
            self.ext_ram_enabled = value & 0xF == 0xA;
        } else if (0x2000..0x4000).contains(&address) {
            // TODO: rom bank number upper bits
            let rom_bank_number = value & 0x1F;
            if rom_bank_number == 0 {
                self.rom_bank_number = 1;
            } else {
                self.rom_bank_number = rom_bank_number as usize; // TODO: needs to be masked to log2(#banks)
            }
        } else if (0x4000..0x6000).contains(&address) {
            let ram_bank_number = value & 0x3;
            self.ram_bank_number = ram_bank_number as usize; // TODO: needs to be checked for number of ram
                                                             // banks
        } else if map::EXT_RAM.contains(&address) && self.ext_ram_enabled {
            self.memory[address] = value;
        }
    }

    fn read_range(&self, memory_range: MemoryRange) -> Vec<u8> {
        memory_range
            .into_iter()
            .map(|address| self.read(address))
            .collect()
    }
}
