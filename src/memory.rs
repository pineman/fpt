use std::ops::Range;

pub type Address = u16;
pub type MemoryRange = Range<Address>;

/// You can access these consts like this:
/// ```
/// assert_eq!(fpt::memory::map::ROM_DATA.start, 0x0100);
/// ```
pub mod map {
    use super::{Address, MemoryRange};

    /// This is where the bootrom lives
    pub const BOOTROM: MemoryRange = 0x0000..0x0100;

    /// The Cartridge Header
    pub const ROM_DATA: MemoryRange = 0x0100..0x0150;

    /// User Program Area (32 KB)
    pub const USER_PROGRAM: MemoryRange = 0x0150..0x8000;

    /// Video RAM (8 KB)
    pub const VRAM: MemoryRange = 0x8000..0xA000;

    /// External Expansion Working RAM (8 KB)
    pub const EXT_WRAM: MemoryRange = 0xA000..0xC000;

    /// Unit Working RAM (8 KB)
    pub const WRAM: MemoryRange = 0xC000..0xE000;

    /// Object Attribute Memory (40 OBJs, 40 x 32 bits)
    pub const OAM: MemoryRange = 0xFE00..0xFEA0;

    /// Port/Mode Registers, Control Registers, Sound Registers
    pub const MANY_REGISTERS: MemoryRange = 0xFF00..0xFF80;

    /// Working & Stack RAM (127 bytes)
    pub const HRAM: MemoryRange = 0xFF80..0xFFFF;

    /// Address used to turn the interrupt system on or off
    pub const INTERRUPT_SWITCH: Address = 0xFFFF;
}

#[derive(Clone)]
pub struct Bus {
    mem: [u8; 65536],
}

impl PartialEq for Bus {
    fn eq(&self, other: &Self) -> bool {
        self.slice(map::WRAM) == other.slice(map::WRAM)
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Self {
        Self { mem: [0; 65536] }
    }

    pub fn read(&self, address: Address) -> u8 {
        self.mem[address as usize]
    }

    pub fn write(&mut self, address: Address, value: u8) {
        self.mem[address as usize] = value;
    }

    pub fn slice(&self, range: MemoryRange) -> &[u8] {
        &self.mem[(range.start as usize)..(range.end as usize)]
    }

    pub fn mut_slice(&mut self, range: MemoryRange) -> &mut [u8] {
        &mut self.mem[(range.start as usize)..(range.end as usize)]
    }

    pub fn each_byte(&self) -> std::iter::Enumerate<std::array::IntoIter<u8, 65536>> {
        self.mem.into_iter().enumerate()
    }

    pub fn load_bootrom(&mut self, bootrom: &[u8; 256]) {
        self.mem[..map::BOOTROM.end as usize].clone_from_slice(bootrom);
    }
}
