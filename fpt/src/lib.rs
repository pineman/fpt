#![feature(bigint_helper_methods)]
#![feature(exclusive_range_pattern)]
#![feature(iter_intersperse)]
#![feature(array_chunks)]

mod bitwise;
pub mod debugger;
pub mod lr35902;
pub mod memory;
pub mod ppu;

use lr35902::LR35902;
use memory::Bus;
use ppu::{Ppu, Frame};

pub struct Gameboy {
    bus: Bus,
    cpu: LR35902,
    ppu: Ppu,
}

impl Gameboy {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::new_with_hook(Box::new(|_frame: ppu::Frame| {}))
    }

    pub fn new_with_zmq() -> Self {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::PUB).unwrap();
        socket.bind("tcp://127.0.0.1:5000").unwrap();

        Self::new_with_hook(Box::new(move |frame: ppu::Frame| {
            let message = zmq::Message::from(frame.to_vec());
            socket.send("frame", zmq::SNDMORE).unwrap();
            socket.send(message, 0).unwrap();
        }))
    }

    fn new_with_hook(frame_hook: Box<dyn Fn(Frame)->()>) -> Self {
        let bus = Bus::new();
        Self {
            bus: bus.clone(),
            cpu: LR35902::new(bus.clone()),
            ppu: Ppu::new(
                bus,
                frame_hook,
            ),
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

    pub fn step(&mut self) -> u32 {
        let cycles = self.cpu.step() as u32;
        self.ppu.step(cycles);
        cycles
    }

    pub fn get_frame(&self) -> &ppu::Frame {
        self.ppu.get_frame()
    }
}
