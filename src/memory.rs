/// You can access these consts like this:
/// ```
/// assert_eq!(fpt::memory::map::ROM_DATA.start, 0x0100);
/// ```
pub mod map {
    use std::ops::Range;
    pub type Address = u16;
    pub type MemoryRange = Range<Address>;

    /// ROM Data Area
    pub const ROM_DATA: MemoryRange = 0x0100..0x0150;

    /// User Program Area (32 KB)
    pub const USER_PROGRAM: MemoryRange = 0x0150..0x8000;

    /// External Expansion Working RAM (8 KB)
    pub const EXT_WRAM: MemoryRange = 0xA000..0xC000;

    /// Unit Working RAM (8 KB)
    pub const WRAM: MemoryRange = 0xC000..0xE000;

    /// Object Attribute Memory (40 OBJs, 40 x 32 bits)
    pub const OAM: MemoryRange = 0xFE00..0xFEA0;

    /// Port/Mode Registers, Control Registers, Sound Registers
    pub const MANY_REGISTERS: MemoryRange = 0xFF00..0xFF80;

    /// Working & Stack RAM (127 bytes)
    pub const STACK_RAM: MemoryRange = 0xFF80..0xFFFE;

    /// "High RAM"? Correct me if I'm wrong
    pub const HRAM: MemoryRange = 0xFFFE..0xFFFF;
}
