use std::cell::{RefCell, RefMut};
use std::ops::Range;
use std::rc::Rc;

pub type Address = usize;
pub type MachineAddress = u16;
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

    pub const LCDC: Address = 0xFF40;
    pub const STAT: Address = 0xFF41;

    pub const SCY: Address = 0xFF42;

    pub const SCX: Address = 0xFF43;

    pub const LY: Address = 0xFF44;

    pub const LYC: Address = 0xFF45;

    /// Working & Stack RAM (127 bytes)
    pub const HRAM: MemoryRange = 0xFF80..0xFFFF;

    /// Address used to turn the interrupt system on or off
    pub const INTERRUPT_SWITCH: Address = 0xFFFF;
}

#[derive(Clone)]
pub struct Memory {
    mem: [u8; 65536],
    cartridge: Vec<u8>,
    bootrom: [u8; 256],
}

impl PartialEq for Memory {
    fn eq(&self, other: &Self) -> bool {
        self.slice(map::WRAM) == other.slice(map::WRAM)
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; 65536],
            cartridge: Vec::new(),
            bootrom: [0; 256],
        }
    }

    pub fn slice(&self, range: MemoryRange) -> &[u8] {
        &self.mem[range.start..range.end]
    }

    pub fn slice_mut(&mut self, range: MemoryRange) -> &mut [u8] {
        &mut self.mem[range.start..range.end]
    }
}

#[derive(Clone, PartialEq)]
pub struct Bus(Rc<RefCell<Memory>>);

impl Bus {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Bus(Rc::new(RefCell::new(Memory::new())))
    }

    pub fn memory(&self) -> RefMut<Memory> {
        self.0.borrow_mut()
    }

    pub fn load_bootrom(&mut self, bootrom: &[u8; 256]) {
        self.memory().bootrom.clone_from_slice(bootrom);
        self.clone_from_slice(map::BOOTROM, bootrom);
    }

    pub fn load_cartridge(&mut self, cartridge: &Vec<u8>) {
        if cartridge.len() < 0x8000 {
            println!("This is not a  rom, fuck you!");
            panic!();
        }
        self.memory().cartridge = cartridge.to_vec();
        self.clone_from_slice(0x100..0x8000, &cartridge[0x100..cartridge.len()]);
    }

    pub fn read(&self, address: MachineAddress) -> u8 {
        self.memory().mem[address as Address]
    }

    pub fn write(&mut self, address: MachineAddress, value: u8) {
        self.memory().mem[address as Address] = value;
    }

    fn _read(&self, address: Address) -> u8 {
        self.memory().mem[address]
    }

    fn _write(&mut self, address: Address, value: u8) {
        self.memory().mem[address] = value;
    }

    pub fn clone_from_slice(&mut self, range: MemoryRange, slice: &[u8]) {
        self.memory().mem[range.start..range.end].clone_from_slice(slice);
    }

    //pub fn slice(&self, range: MemoryRange) -> &[u8] {
    //    &self.memory().mem[(range.start as usize)..(range.end as usize)]
    //}

    //pub fn mut_slice(&mut self, range: MemoryRange) -> &mut [u8] {
    //    &mut self.memory().mem[(range.start as usize)..(range.end as usize)]
    //}

    pub fn each_byte(&self) -> std::iter::Enumerate<std::array::IntoIter<u8, 65536>> {
        self.memory().mem.into_iter().enumerate()
    }

    // registers
    pub fn lcdc(&self) -> u8 {
        self._read(map::LCDC)
    }

    pub fn set_lcdc(&mut self, value: u8) {
        self._write(map::LCDC, value);
    }

    pub fn stat(&self) -> u8 {
        self._read(map::LCDC)
    }

    pub fn set_stat(&mut self, value: u8) {
        self._write(map::STAT, value);
    }

    pub fn scy(&self) -> u8 {
        self._read(map::SCY)
    }

    pub fn set_scy(&mut self, value: u8) {
        self._write(map::SCY, value);
    }

    pub fn scx(&self) -> u8 {
        self._read(map::SCX)
    }

    pub fn set_scx(&mut self, value: u8) {
        self._write(map::SCX, value);
    }

    pub fn ly(&self) -> u8 {
        self._read(map::LY)
    }

    pub fn set_ly(&mut self, value: u8) {
        self._write(map::LY, value);
    }

    pub fn lyc(&self) -> u8 {
        self._read(map::LYC)
    }

    pub fn set_lyc(&mut self, value: u8) {
        self._write(map::LYC, value)
    }

    pub fn vram(&self) -> Vec<u8> {
        self.memory().mem[map::VRAM].to_vec()
    }
}
