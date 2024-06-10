#![feature(bigint_helper_methods)]
#![feature(exclusive_range_pattern)]
#![feature(array_chunks)]
#![feature(iter_intersperse)]
#![feature(new_uninit)]
#![feature(ptr_as_uninit)]

use lr35902::LR35902;
use memory::Bus;
use ppu::{Frame, Ppu, DOTS_IN_ONE_FRAME};

pub mod bitwise;
pub mod lr35902;
pub mod memory;
pub mod ppu;

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

    pub fn unsafely_optimized_new() -> Self {
        let bus = Bus::unsafely_optimized_new();
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

    pub fn ppu(&self) -> &Ppu {
        &self.ppu
    }
    pub fn ppu_mut(&mut self) -> &mut Ppu {
        &mut self.ppu
    }

    pub fn instruction(&mut self) -> u32 {
        let cycles = self.cpu.instruction() as u32;
        // TODO: care for double speed mode (need to run half as much dots)
        self.ppu.step(cycles);
        cycles
    }

    pub fn advance_frame(&mut self) -> &Frame {
        for _ in 0..DOTS_IN_ONE_FRAME {
            // TODO: care for double speed mode (need to run two cpu t_cycles)
            self.cpu.t_cycle();
            self.ppu.step(1);
        }
        self.ppu.get_frame()
    }

    pub fn get_frame(&self) -> &Frame {
        self.ppu.get_frame()
    }

    pub fn cycles_in_one_frame(&self) -> u32 {
        // TODO: care for double speed mode
        DOTS_IN_ONE_FRAME
    }
}
