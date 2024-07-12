use std::cell::RefCell;
use std::rc::Rc;

use crate::memory::map;
use crate::memory::{Address, MemoryRange};

fn get_rom_size(tape: &[u8]) -> u8 {
    tape[map::ROM_SIZE]
}

fn get_ram_size(tape: &[u8]) -> u8 {
    tape[map::RAM_SIZE]
}

fn get_cartridge_type(tape: &[u8]) -> u8 {
    tape[map::CARTRIDGE_FLAG]
}

fn convert_rom_size(rom_size: u8) -> usize {
    match rom_size {
        0x00 => 2,
        0x01 => 4,
        0x02 => 8,
        0x03 => 16,
        0x04 => 32,
        0x05 => 64,
        0x06 => 128,
        0x07 => 256,
        0x08 => 512,
        0x52 => 72,
        0x53 => 80,
        0x54 => 96,
        _ => panic!(),
    }
}

fn convert_ram_size(ram_size: u8) -> u16 {
    match ram_size {
        0x00 => 0,
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => panic!(),
    }
}

pub trait Cartridge {
    fn read(&self, address: Address) -> u8;
    fn write(&mut self, address: Address, value: u8);

    fn read_range(&self, memory_range: MemoryRange) -> Vec<u8>;

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
        self.read(map::CARTRIDGE_FLAG)
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

    fn read_range(&self, memory_range: MemoryRange) -> Vec<u8> {
        Vec::new()
    }
}

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

pub struct Mbc1Cartridge {
    memory: Vec<u8>,
    rom_banks: Vec<[u8; 0x4000]>,
    ram_banks: Vec<[u8; 0x2000]>,
    ext_ram_enabled: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,
}

impl Mbc1Cartridge {
    pub fn new(cartridge: &[u8]) -> Mbc1Cartridge {
        let rom_size = dbg!(convert_rom_size(get_rom_size(cartridge)));
        let ram_size = dbg!(convert_ram_size(get_ram_size(cartridge)));
        let mut rom_banks = vec![[0; 0x4000]; dbg!(rom_size as usize)];
        let mut ram_banks = vec![[0; 0x2000]; dbg!(ram_size as usize)];

        // TODO: wtf is this initialization
        for i in 0..rom_size {
            for j in 0..0x4000 {
                let i = i as usize;
                let j = j as usize;
                rom_banks[i][j] = cartridge[0x4000 * i + j];
            }
        }

        for i in 0..ram_size {
            for j in 0..0x2000 {
                let i = i as usize;
                let j = j as usize;
                ram_banks[i][j] = cartridge[0x2000 * i + j];
            }
        }

        Mbc1Cartridge {
            memory: cartridge.to_vec(),
            rom_banks,
            ram_banks,
            ext_ram_enabled: false,
            rom_bank_number: 0,
            ram_bank_number: 0,
        }
    }
}

impl Cartridge for Mbc1Cartridge {
    fn read(&self, address: Address) -> u8 {
        if map::EXT_RAM.contains(&address) && !self.ext_ram_enabled {
            0 // TODO: check that disabled ram reads 0
        } else if map::EXT_RAM.contains(&address) && self.ext_ram_enabled {
            self.ram_banks[self.ram_bank_number as usize][address - map::EXT_RAM.start]
        } else if map::ROM_BANK0.contains(&address) {
            self.rom_banks[0][address - map::ROM_BANK0.start]
        } else if map::ROM_BANK1.contains(&address) {
            self.rom_banks[self.rom_bank_number as usize][address - map::ROM_BANK1.start]
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
        self.memory[memory_range].to_vec()
    }
}

pub fn create_memory_bank(cartridge_data: &[u8]) -> Rc<RefCell<dyn Cartridge>> {
    let cartridge_type = get_cartridge_type(cartridge_data);

    match dbg!(cartridge_type) {
        0x00 => Rc::new(RefCell::new(NoMbcCartridge::new(cartridge_data))),
        0x01 | 0x02 | 0x03 => Rc::new(RefCell::new(Mbc1Cartridge::new(cartridge_data))),
        _ => panic!(),
    }
}