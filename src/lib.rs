#![feature(bigint_helper_methods)]
#![feature(exclusive_range_pattern)]
#![feature(iter_intersperse)]

mod bitwise;
pub mod lr35902;
pub mod memory;
pub mod ppu;

pub mod debugger;
pub use debugger::DebuggerTextInterface;

use crate::lr35902::LR35902;
use crate::ppu::Ppu;

use memory::Bus;
pub struct Gameboy {
    bus: Bus,
    cpu: LR35902,
    ppu: Ppu,
}

impl Gameboy {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let bus = Bus::new();
        Self {
            bus: bus.clone(),
            cpu: LR35902::new(bus.clone()),
            ppu: Ppu::new(bus),
        }
    }

    pub fn set_debug(&mut self, enabled: bool) {
        self.cpu.set_debug(enabled);
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        self.bus.load_cartridge(rom);
    }

    pub fn cpu(&self) -> &LR35902 {
        &self.cpu
    }

    pub fn step(&mut self) -> u8 {
        let cycles = self.cpu.step();
        self.ppu.step(cycles);
        cycles
    }

    pub fn get_frame(&self) -> &ppu::Frame {
        self.ppu.get_frame()
    }
}
