use std::cell::{Ref, RefCell, RefMut};
use std::ops::DerefMut;
use std::ops::Range;
use std::rc::Rc;

use crate::bw;
use crate::memory::map;
use crate::memory::{create_memory_bank, Cartridge, EmptyCartridge, NoMbcCartridge};

pub type Address = usize;
pub type MemoryRange = Range<Address>;

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

#[derive(Clone)]
pub struct Memory {
    mem: Vec<u8>,
    cartridge: Rc<RefCell<dyn Cartridge>>,
    bootrom: &'static [u8; 256],
    rom_first256bytes: Vec<u8>,
    code_listing: Vec<Option<String>>,
    pub buttons: Buttons,
    bootrom_unloaded: bool,
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
            cartridge: Rc::new(RefCell::new(EmptyCartridge::new())),
            bootrom: include_bytes!("../../dmg.bin"),
            rom_first256bytes: vec![0; 256],
            code_listing: vec![ARRAY_REPEAT_VALUE; 0xffff + 1],
            buttons: Buttons::default(),
            bootrom_unloaded: false,
        }
    }

    pub fn cartridge(&self) -> &Rc<RefCell<dyn Cartridge>> {
        &self.cartridge
    }

    pub fn cartridge_mut(&mut self) -> &mut Rc<RefCell<dyn Cartridge>> {
        &mut self.cartridge
    }

    pub fn set_cartridge(&mut self, cartridge: Rc<RefCell<dyn Cartridge>>) {
        self.cartridge = cartridge;
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

    pub fn unload_bootrom(&mut self) {
        self.bootrom_unloaded = true;
    }

    pub fn bootrom_unloaded(&self) -> bool {
        self.bootrom_unloaded
    }
}

#[derive(Clone, PartialEq)]
pub struct Bus(Rc<RefCell<Memory>>);

impl Bus {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Bus(Rc::new(RefCell::new(Memory::new())))
    }

    pub fn memory(&self) -> Ref<Memory> {
        self.0.borrow()
    }

    pub fn memory_mut(&self) -> RefMut<Memory> {
        self.0.borrow_mut()
    }

    pub fn load_bootrom(&mut self) {
        self.memory_mut().rom_first256bytes = self.copy_range(0x0000..0x0100);
        let bootrom = self.memory().bootrom;
        self.clone_from_slice(map::BOOTROM, bootrom);
        self.memory_mut().code_listing[map::BOOTROM].fill(None);
    }

    pub fn unload_bootrom(&mut self) {
        self.memory_mut().unload_bootrom();
        // TODO: unload?
        //let backup = self.memory_mut().rom_first256bytes.clone();
        //self.clone_from_slice(map::BOOTROM, &backup);
        //self.memory_mut().code_listing[map::BOOTROM].fill(None);
    }

    pub fn load_cartridge(&mut self, cartridge: &[u8]) {
        // TODO: load
        self.memory_mut()
            .set_cartridge(create_memory_bank(cartridge));

        //println!("title: {}", self.memory().cartridge().get_title());
        //println!("code: {}", self.memory().cartridge().get_manufacturer_code());
        //println!("new licensee code: {}", self.memory().cartridge().get_new_licensee_code());
        //println!("cartridge type: {}", self.memory().cartridge().get_cartridge_type());
        //println!("rom size: {}", self.memory().cartridge().get_rom_size());
        //println!("ram size: {}", self.memory().cartridge().get_ram_size());
        //println!("licensee code: {}", self.memory().cartridge().get_old_licensee_code());
        //println!("version number: {}", self.memory().cartridge().get_version_number());

        //panic!();
    }

    pub fn read(&self, address: Address) -> u8 {
        if map::BOOTROM.contains(&address) && !self.memory().bootrom_unloaded() {
            self.memory().bootrom[address]
        } else if map::ROM_BANK0.contains(&address)
            || map::ROM_BANK1.contains(&address)
            || map::EXT_RAM.contains(&address)
        {
            self.memory().cartridge().borrow().read(address)
        } else if address == map::JOYP {
            self.joyp()
        } else {
            self.memory().mem[address as Address]
        }
    }

    pub fn write(&mut self, address: Address, value: u8) {
        if map::ROM_BANK0.contains(&address)
            || map::ROM_BANK1.contains(&address)
            || map::EXT_RAM.contains(&address)
        {
            self.memory_mut()
                .cartridge_mut()
                .borrow_mut()
                .write(address, value);
        } else {
            self.memory_mut().mem[address as Address] = value;
        }
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

    pub fn clone_from_slice(&mut self, mut range: MemoryRange, slice: &[u8]) {
        dbg!(slice.len());
        dbg!(&range);

        if range.end > 65535 {
            range.end = 65535;
        }

        let slice = slice.to_vec();

        let slice = &slice[0..(range.end - range.start)];
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
