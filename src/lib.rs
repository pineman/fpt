#![feature(bigint_helper_methods)]

mod bitwise;
pub mod lr35902;
pub mod memory;
mod ppu;

use crate::lr35902::LR35902;
use crate::memory::Bus;
use crate::ppu::Ppu;

pub struct Gameboy {
    cpu: LR35902,
    ppu: Ppu,
}

impl Gameboy {
    pub fn new() -> Self {
        let bus = Box::new(Bus::new());
        Self {
            cpu: LR35902::new(bus.clone()),
            ppu: Ppu::new(bus),
        }
    }

    pub fn step(&mut self) {
        self.cpu.step();
        self.ppu.render(&mut self.cpu);
    }
}
