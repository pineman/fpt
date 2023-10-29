#![feature(bigint_helper_methods)]
#![feature(exclusive_range_pattern)]
#![feature(iter_intersperse)]

mod bitwise;
pub mod debugger;
pub mod lr35902;
pub mod memory;
pub mod ppu;

use lr35902::LR35902;
use memory::Bus;
use ppu::Ppu;

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

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        self.bus.load_cartridge(rom);
    }

    pub fn cpu(&self) -> &LR35902 {
        &self.cpu
    }

    pub fn cpu_mut(&mut self) -> &mut LR35902 {
        &mut self.cpu
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
