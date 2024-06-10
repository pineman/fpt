use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut, Range};
use std::ptr;
use std::rc::Rc;

pub type Address = usize;
pub type GBAddress = u16;
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
    pub const P1: Address = 0xFF00;
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
    address_space: [u8; 65536],
    cartridge: Vec<u8>,
    bootrom: &'static [u8; 256],
    code_listing: [Option<String>; 0xffff + 1],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

/// Makes Memory act like the underlying `address_space` slice when calling
/// methods and indexing and slicing with `[]`, making the following equivalent:
///
///   - `mem1[map::WRAM] == mem2[map::WRAM]`
///   - `mem1.address_space[map::WRAM] == mem2.address_space[map::WRAM]`
///
/// This should make most of the helper methods in `Memory` and `Bus` redundant.
///
/// This also exploses `address_space` outside this module, so anyone with a
/// reference to `Memory` can do whatever slice operations they want on
/// `address_space`. For example, here's `LR35902::mem8` directly reading a byte
/// from `address_space` (adapted to doctest):
///
///     pub fn mem8(self_mem: &fpt::memory::Bus, index: u16) -> u8 {
///         self_mem.memory()[index as fpt::memory::Address]
///     }
impl Deref for Memory {
    type Target = [u8; 65536];

    fn deref(&self) -> &Self::Target {
        return &self.address_space;
    }
}

/// In combination with implementing `Deref`, this further allows the following:
///
///   - `mem.fill(0)`
///   - `mem[map::BOOTROM].clone_from_slice(mem.bootrom)`
impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.address_space;
    }
}

/// The memories' whole address space is compared bit by bit
/// TODO or maybe this is too much?
impl PartialEq for Memory {
    fn eq(&self, other: &Self) -> bool {
        self.address_space[..] == other.address_space[..]
    }
}

impl Memory {
    const BOOTROM: &'static [u8; 256] = include_bytes!("../dmg0.bin");

    pub fn new() -> Self {
        const ARRAY_REPEAT_VALUE: Option<String> = None;
        Self {
            address_space: [0; 65536],
            cartridge: Vec::new(),
            bootrom: Self::BOOTROM,
            code_listing: [ARRAY_REPEAT_VALUE; 0xffff + 1],
        }
    }

    /// Takes a pointer to zero-initialized memory and manually initializes
    /// `Memory` fields that shouldn't remain zero-initialized.
    ///
    /// # Safety
    ///
    /// idk. This function was made to be caled from `Bus::unsafely_optmized_new()`.
    /// And we're only calling `Bus::unsafely_optmized_new()` from tests, right?
    /// (Clippy is forcing me to write this `# Safety` section, so here you are)
    pub unsafe fn initialize_from_zero(ptr_to_mem: *mut Memory) {
        // Writes a properly initialized Vec to ptr_to_mem->cartridge without
        // dropping a zero-initialized Vec, which would be undefined behaviour
        let ptr_to_cartridge = ptr::addr_of_mut!((*ptr_to_mem).cartridge);
        ptr::write(ptr_to_cartridge, Vec::new());

        // Point ptr_to_mem->bootrom (a dangling reference if zero-initialized) to BOOTROM
        (*ptr_to_mem).bootrom = Self::BOOTROM;

        // It *might* be fine to leave code_listing: [Option<String>; 0xffff + 1] zero-initialized.
        // None, being the first variant in the Option enum, should have discriminant = 0, I guess.
    }

    pub fn array_ref<const N: usize>(&self, from: Address) -> &[u8; N] {
        self.address_space[from..from + N].try_into().unwrap() // guaranteed to have size N
    }

    pub fn code_listing(&self) -> &[Option<String>; 0xffff + 1] {
        &self.code_listing
    }

    pub fn set_code_listing_at(&mut self, pc: u16, v: String) {
        self.code_listing[pc as usize] = Some(v);
    }
}

/// An Rc-RefCell wrapper around Memory that allows it to be shared by the CPU, the PPU, etc.
#[derive(Clone, PartialEq)]
pub struct Bus(Rc<RefCell<Memory>>);

/// Constructors
impl Bus {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Bus(Rc::new(RefCell::new(Memory::new())))
    }

    pub fn unsafely_optimized_new() -> Self {
        let rc = Rc::<RefCell<Memory>>::new_zeroed();
        let rc = unsafe {
            let ptr_to_zeroed_memory = (*rc.as_ptr()).as_ptr();
            Memory::initialize_from_zero(ptr_to_zeroed_memory);
            rc.assume_init()
        };

        Bus(rc)
    }
}

/// Accessors
impl Bus {
    pub fn memory(&self) -> Ref<Memory> {
        self.0.borrow()
    }

    pub fn memory_mut(&self) -> RefMut<Memory> {
        self.0.borrow_mut()
    }

    /// Immutably borrow the inner `Memory` in a scoped way.
    pub fn borrow<R>(&self, borrower: impl FnOnce(&Memory) -> R) -> R {
        borrower(&self.0.borrow()) // reborrow
    }

    /// Mutably borrow the inner `Memory` in a scoped way.
    pub fn borrow_mut<R>(&self, borrower: impl FnOnce(&mut Memory) -> R) -> R {
        borrower(&mut self.0.borrow_mut()) // reborrow
    }

    /// Do something with the VRAM
    pub fn with_vram<R>(&self, reader: impl FnOnce(&[u8]) -> R) -> R {
        self.borrow(|mem| reader(&mem.address_space[map::VRAM]))
    }
}

/// Operations
impl Bus {
    pub fn load_bootrom(&mut self) {
        let mem = &mut *self.0.borrow_mut();
        mem.address_space[map::BOOTROM].clone_from_slice(mem.bootrom);
        mem.code_listing[map::BOOTROM].fill(None);
    }

    pub fn unload_bootrom(&mut self) {
        let mem = &mut *self.0.borrow_mut();
        mem.address_space[map::BOOTROM].clone_from_slice(&mem.cartridge[map::BOOTROM]);
    }

    pub fn load_cartridge(&mut self, cartridge: &[u8]) {
        const CARTRIDGE_AREA: MemoryRange = 0x0100..0x8000;
        let mem = &mut *self.0.borrow_mut();
        mem.address_space[CARTRIDGE_AREA].clone_from_slice(&cartridge[CARTRIDGE_AREA]);
    }

    pub fn read(&self, address: GBAddress) -> u8 {
        self.memory_mut().address_space[address as Address]
    }

    pub fn write(&mut self, address: GBAddress, value: u8) {
        self.memory_mut().address_space[address as Address] = value;
    }

    pub fn copy_range(&self, range: MemoryRange) -> Vec<u8> {
        self.memory_mut().address_space[range.start..range.end].to_vec()
    }

    pub fn with_slice<T>(&self, range: MemoryRange, reader: impl FnOnce(&[u8]) -> T) -> T {
        reader(&self.memory().address_space[range])
    }

    /// Runs closure `reader` with access to a fixed-size slice of `N` bytes.
    pub fn with_span<const N: usize, T>(
        &self,
        start: Address,
        reader: impl FnOnce(&[u8; N]) -> T,
    ) -> T {
        reader(self.memory().array_ref(start))
    }

    pub fn each_byte(&self) -> std::iter::Enumerate<std::array::IntoIter<u8, 65536>> {
        self.memory_mut().address_space.into_iter().enumerate()
    }
}

/// Register accessors
impl Bus {
    fn _read(&self, address: Address) -> u8 {
        self.memory_mut().address_space[address]
    }

    fn _write(&mut self, address: Address, value: u8) {
        self.memory_mut().address_space[address] = value;
    }

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
}
