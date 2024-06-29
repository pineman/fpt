use std::cell::{Ref, RefCell, RefMut};
use std::ops::Range;
use std::rc::Rc;

use crate::bw;

pub type Address = usize;
pub type MemoryRange = Range<Address>;

/// You can access these consts like this:
/// ```
/// assert_eq!(fpt::memory::map::ROM_DATA.start, 0x0100);
/// ```
pub mod map {
    use super::{Address, MemoryRange};

    //-------------------------------------------------------------------------
    // Memory map
    //-------------------------------------------------------------------------

    /// This is where the bootrom lives
    pub const BOOTROM: MemoryRange = 0x0000..0x0100;

    /// The Cartridge Header
    pub const ROM_DATA: MemoryRange = 0x0100..0x0150;

    /// User Program Area (32 KB)
    /// 0x0000..0x4000 From cartridge, usually a fixed bank
    /// 0x4000..0x8000 From cartridge, switchable bank via mapper (if any) pub const USER_PROGRAM: MemoryRange = 0x0000..0x8000;

    /// Video RAM (8 KB) - In CGB mode, switchable bank 0/1
    pub const VRAM: MemoryRange = 0x8000..0xA000;

    /// External Expansion Working RAM (8 KB) - From cartridge, switchable bank if any
    pub const EXT_WRAM: MemoryRange = 0xA000..0xC000;

    /// Unit Working RAM (8 KB)
    pub const WRAM: MemoryRange = 0xC000..0xE000;

    /// Not usable (Mirror of C000~DDFF (ECHO RAM)) https://gbdev.io/pandocs/Memory_Map.html#echo-ram
    pub const NOT_USABLE1: MemoryRange = 0xE000..0xFE00;

    /// Object Attribute Memory (40 OBJs, 40 x 32 bits)
    pub const OAM: MemoryRange = 0xFE00..0xFEA0;

    /// Not usable https://gbdev.io/pandocs/Memory_Map.html#fea0-feff-range
    pub const NOT_USABLE2: MemoryRange = 0xFEA0..0xFF00;

    //-------------------------------------------------------------------------
    // I/O Registers
    //-------------------------------------------------------------------------

    /// Joypad
    pub const JOYP: Address = 0xFF00;
    /// Serial transfer data
    pub const SB: Address = 0xFF01;
    /// Serial transfer control
    pub const SC: Address = 0xFF02;
    /// Divider register
    pub const DIV: Address = 0xFF04;
    /// Timer counter
    pub const TIMA: Address = 0xFF05;
    /// Timer modulo
    pub const TMA: Address = 0xFF06;
    /// Timer control
    pub const TAC: Address = 0xFF07;

    //-------------------------------------------------------------------------
    // I/O: Sound
    //-------------------------------------------------------------------------

    /// Sound channel 1 sweep
    pub const NR10: Address = 0xFF10;
    /// Sound channel 1 length timer & duty cycle
    pub const NR11: Address = 0xFF11;
    /// Sound channel 1 volume & envelope
    pub const NR12: Address = 0xFF12;
    /// Sound channel 1 period low
    pub const NR13: Address = 0xFF13;
    /// Sound channel 1 period high & control
    pub const NR14: Address = 0xFF14;
    /// Sound channel 2 length timer & duty cycle
    pub const NR21: Address = 0xFF16;
    /// Sound channel 2 volume & envelope
    pub const NR22: Address = 0xFF17;
    /// Sound channel 2 period low
    pub const NR23: Address = 0xFF18;
    /// Sound channel 2 period high & control
    pub const NR24: Address = 0xFF19;
    /// Sound channel 3 DAC enable
    pub const NR30: Address = 0xFF1A;
    /// Sound channel 3 length timer
    pub const NR31: Address = 0xFF1B;
    /// Sound channel 3 output level
    pub const NR32: Address = 0xFF1C;
    /// Sound channel 3 period low
    pub const NR33: Address = 0xFF1D;
    /// Sound channel 3 period high & control
    pub const NR34: Address = 0xFF1E;
    /// Sound channel 4 length timer
    pub const NR41: Address = 0xFF20;
    /// Sound channel 4 volume & envelope
    pub const NR42: Address = 0xFF21;
    /// Sound channel 4 frequency & randomness
    pub const NR43: Address = 0xFF22;
    /// Sound channel 4 control
    pub const NR44: Address = 0xFF23;
    /// Master volume & VIN panning
    pub const NR50: Address = 0xFF24;
    /// Sound panning
    pub const NR51: Address = 0xFF25;
    /// Sound on/off
    pub const NR52: Address = 0xFF26;
    /// Wave RAM
    pub const WAVE_RAM: MemoryRange = 0xFF30..0xFF40;

    //-------------------------------------------------------------------------
    // IO: PPU
    //-------------------------------------------------------------------------

    /// LCD control
    pub const LCDC: Address = 0xFF40;
    /// LCD status
    pub const STAT: Address = 0xFF41;
    /// Viewport Y position
    pub const SCY: Address = 0xFF42;
    /// Viewport X position
    pub const SCX: Address = 0xFF43;
    /// LCD Y coordinate
    pub const LY: Address = 0xFF44;
    /// LY compare
    pub const LYC: Address = 0xFF45;
    /// OAM DMA source address & start
    pub const DMA: Address = 0xFF46;
    /// BG palette data (DMG)
    pub const BGP: Address = 0xFF47;
    /// OBJ palette 0 data (DMG)
    pub const OBP0: Address = 0xFF48;
    /// OBJ palette 1 data (DMG)
    pub const OBP1: Address = 0xFF49;
    /// Window Y position
    pub const WY: Address = 0xFF4A;
    /// Window X position plus 7
    pub const WX: Address = 0xFF4B;

    /// BANK register: Set to non-zero to disable boot ROM
    pub const BANK: Address = 0xFF50;

    //-------------------------------------------------------------------------
    // CGB extra
    // https://gbdev.io/pandocs/CGB_Registers.html
    //-------------------------------------------------------------------------

    /// Prepare speed switch (CGB)
    pub const KEY1: Address = 0xFF4C;
    /// VRAM bank (CGB)
    pub const VBK: Address = 0xFF4F;
    /// VRAM DMA source high (CGB)
    pub const HDMA1: Address = 0xFF51;
    /// VRAM DMA source low (CGB)
    pub const HDMA2: Address = 0xFF52;
    /// VRAM DMA destination high (CGB)
    pub const HDMA3: Address = 0xFF53;
    /// VRAM DMA destination low (CGB)
    pub const HDMA4: Address = 0xFF54;
    /// VRAM DMA length/mode/start (CGB)
    pub const HDMA5: Address = 0xFF55;
    /// Infrared communications port (GGB)
    pub const RP: Address = 0xFF56;
    /// Background color palette specification / Background palette index (CGB)
    pub const BCPS: Address = 0xFF68;
    /// Background color palette data / Background palette data (CGB)
    pub const BCPD: Address = 0xFF69;
    /// OBJ color palette specification / OBJ palette index (CGB)
    pub const OCPS: Address = 0xFF6A;
    /// OBJ color palette data / OBJ palette data (CGB)
    pub const OCPD: Address = 0xFF6B;
    /// Object priority mode (CGB)
    pub const OPRI: Address = 0xFF6C;
    /// WRAM bank (CGB) pub const SVBK: Address = 0xFF70;
    /// Audio digital outputs 1 & 2 (CGB)
    pub const PCM12: Address = 0xFF76;
    /// Audio digital outputs 3 & 4 (CGB)
    pub const PCM34: Address = 0xFF77;

    //-------------------------------------------------------------------------
    // High RAM
    //-------------------------------------------------------------------------

    /// Working & Stack RAM (127 bytes)
    pub const HRAM: MemoryRange = 0xFF80..0xFFFF;

    //-------------------------------------------------------------------------
    // Interrupts
    //-------------------------------------------------------------------------

    /// Interrupt enable
    pub const IE: Address = 0xFFFF;
    /// Interrupt flag
    pub const IF: Address = 0xFF0F;
}

#[derive(Clone)]
pub struct Memory {
    mem: Vec<u8>,
    cartridge: Vec<u8>,
    bootrom: &'static [u8; 256],
    code_listing: Vec<Option<String>>,
    pub buttons: Buttons,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Buttons {
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
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
        const ARRAY_REPEAT_VALUE: Option<String> = None;
        Self {
            mem: vec![0; 65536],
            cartridge: Vec::new(),
            bootrom: include_bytes!("../dmg.bin"),
            code_listing: vec![ARRAY_REPEAT_VALUE; 0xffff + 1],
            buttons: Buttons::default(),
        }
    }

    pub fn array_ref<const N: usize>(&self, from: Address) -> &[u8; N] {
        self.mem[from..from + N].try_into().unwrap() // guaranteed to have size N
    }

    pub fn slice(&self, range: MemoryRange) -> &[u8] {
        &self.mem[range]
    }

    pub fn slice_mut(&mut self, range: MemoryRange) -> &mut [u8] {
        &mut self.mem[range]
    }

    pub fn code_listing(&self) -> &[Option<String>] {
        &self.code_listing
    }

    pub fn set_code_listing_at(&mut self, pc: u16, v: String) {
        self.code_listing[pc as usize] = Some(v);
    }
}

#[derive(Clone, PartialEq)]
pub struct Bus(Rc<RefCell<Memory>>);

impl Bus {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut bus = Bus(Rc::new(RefCell::new(Memory::new())));
        bus.load_bootrom();
        bus
    }

    pub fn memory(&self) -> Ref<Memory> {
        self.0.borrow()
    }

    pub fn memory_mut(&self) -> RefMut<Memory> {
        self.0.borrow_mut()
    }

    pub fn load_bootrom(&mut self) {
        let bootrom = self.memory().bootrom;
        self.clone_from_slice(map::BOOTROM, bootrom);
        self.memory_mut().code_listing[map::BOOTROM].fill(None)
    }

    pub fn unload_bootrom(&mut self) {
        let cartridge = self.memory_mut().cartridge[map::BOOTROM].to_vec();
        self.clone_from_slice(map::BOOTROM, &cartridge);
        for i in map::BOOTROM {
            self.memory_mut().code_listing[i] = None;
        }
    }

    pub fn load_cartridge(&mut self, cartridge: &[u8]) {
        self.memory_mut().cartridge = cartridge.to_vec();
        let l = cartridge.len();
        self.clone_from_slice(0x0100..l, &cartridge[0x0100..l]);
    }

    pub fn read(&self, address: Address) -> u8 {
        if address == map::JOYP {
            self.joyp()
        } else {
            self.memory().mem[address as Address]
        }
    }

    pub fn write(&mut self, address: Address, value: u8) {
        self.memory_mut().mem[address as Address] = value;
    }

    fn _read(&self, address: Address) -> u8 {
        if address == map::JOYP {
            self.joyp()
        } else {
            self.memory().mem[address]
        }
    }

    fn _write(&mut self, address: Address, value: u8) {
        if address == map::TAC {
            println!("write to TAC: {}", value);
        }
        self.memory_mut().mem[address] = value;
    }

    pub fn clone_from_slice(&mut self, range: MemoryRange, slice: &[u8]) {
        self.memory_mut().mem[range.start..range.end].clone_from_slice(slice);
    }

    pub fn copy_range(&self, range: MemoryRange) -> Vec<u8> {
        self.memory_mut().mem[range.start..range.end].to_vec()
    }

    pub fn with_slice<T>(&self, range: MemoryRange, reader: impl FnOnce(&[u8]) -> T) -> T {
        reader(&self.memory().mem[range])
    }

    /// Runs closure `reader` with access to a fixed-size slice of `N` bytes.
    pub fn with_span<const N: usize, T>(
        &self,
        start: Address,
        reader: impl FnOnce(&[u8; N]) -> T,
    ) -> T {
        reader(self.memory().array_ref(start))
    }

    // registers
    pub fn lcdc(&self) -> u8 {
        self._read(map::LCDC)
    }

    pub fn set_lcdc(&mut self, value: u8) {
        self._write(map::LCDC, value);
    }

    pub fn stat(&self) -> u8 {
        self._read(map::STAT)
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

    pub fn with_vram<R>(&self, reader: impl FnOnce(&[u8]) -> R) -> R {
        reader(&self.memory().mem[map::VRAM])
    }

    fn joyp(&self) -> u8 {
        let buttons = self.buttons();
        let joyp = self.memory().mem[map::JOYP];
        let sel_buttons = !bw::test_bit8::<5>(joyp);
        let sel_dpad = !bw::test_bit8::<4>(joyp);
        let b = if sel_dpad {
            ((buttons.down as u8) << 3)
                + ((buttons.up as u8) << 2)
                + ((buttons.left as u8) << 1)
                + (buttons.right as u8)
        } else if sel_buttons {
            ((buttons.start as u8) << 3)
                + ((buttons.select as u8) << 2)
                + ((buttons.b as u8) << 1)
                + (buttons.a as u8)
        } else {
            0
        };
        (joyp & 0xf0) + (!b & 0x0f)
    }

    pub fn buttons(&self) -> Buttons {
        self.memory().buttons
    }

    pub fn set_buttons(&mut self, buttons: &Buttons) {
        self.memory_mut().buttons = *buttons;
    }

    pub fn ie(&self) -> u8 {
        self._read(map::IE)
    }

    pub fn iflag(&self) -> u8 {
        self._read(map::IF)
    }

    pub fn set_iflag(&mut self, value: u8) {
        self._write(map::IF, value)
    }
}
