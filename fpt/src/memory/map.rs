/// You can access these consts like this:
/// ```
/// assert_eq!(fpt::memory::map::ROM_DATA.start, 0x0100);
/// ```

use super::{Address, MemoryRange};

//-------------------------------------------------------------------------
// Memory map
//-------------------------------------------------------------------------

/// This is where the bootrom lives
pub const BOOTROM: MemoryRange = 0x0000..0x0100;

/// The Cartridge Header
pub const ROM_DATA: MemoryRange = 0x0100..0x0150;

/// User Program Area (32 KB)
/// From cartridge, usually a fixed bank
pub const ROM_BANK0: MemoryRange = 0x0000..0x4000;
/// From cartridge, switchable bank via mapper (if any) pub const USER_PROGRAM: MemoryRange = 0x0000..0x8000;
pub const ROM_BANK1: MemoryRange = 0x4000..0x8000; 

/// Video RAM (8 KB) - In CGB mode, switchable bank 0/1
pub const VRAM: MemoryRange = 0x8000..0xA000;

/// External Expansion Working RAM (8 KB) - From cartridge, switchable bank if any
pub const EXT_RAM: MemoryRange = 0xA000..0xC000;

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

/// Cartridge sections
pub const TITLE: MemoryRange = 0x134..0x143;
pub const MANUFACTURER_CODE: MemoryRange = 0x13F..0x142;
pub const GGB_FLAG: Address = 0x143;
pub const NEW_LICENSEE_CODE: MemoryRange = 0x144..0x145;
pub const SGB_FLAG: Address = 0x146;
pub const CARTRIDGE_FLAG: Address = 0x147;
pub const ROM_SIZE: Address = 0x148;
pub const RAM_SIZE: Address = 0x149;
pub const OLD_LICENSEE_CODE: Address = 0x14b;
pub const VERSION_NUMBER: Address = 0x14c;
