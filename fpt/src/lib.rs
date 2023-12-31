#![feature(bigint_helper_methods)]
#![feature(exclusive_range_pattern)]
#![feature(array_chunks)]
#![feature(iter_intersperse)]

mod bitwise;
pub mod lr35902;
pub mod memory;
pub mod ppu;

use lr35902::LR35902;
use memory::Bus;
use ppu::{Frame, Ppu};

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

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.bus.load_cartridge(rom);
    }

    pub fn bus(&self) -> &Bus {
        &self.bus
    }

    pub fn cpu(&self) -> &LR35902 {
        &self.cpu
    }

    pub fn cpu_mut(&mut self) -> &mut LR35902 {
        &mut self.cpu
    }

    pub fn instruction(&mut self) -> u32 {
        let cycles = self.cpu.instruction() as u32;
        // TODO: care for double speed mode (need to run half as much dots)
        self.ppu.step(cycles);
        cycles
    }

    pub fn cycle(&mut self) {
        // TODO: care for double speed mode (need to run two cpu cycles)
        self.cpu.cycle();
        self.ppu.step(1);
    }

    pub fn frame(&mut self) -> &Frame {
        for _ in 0..70224 {
            self.cpu.cycle();
            self.ppu.step(1);
        }
        self.ppu.get_frame()
    }

    pub fn get_frame(&self) -> &Frame {
        self.ppu.get_frame()
    }
}
