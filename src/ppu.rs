use crate::lr35902::LR35902;
use crate::memory::Bus;

//#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Ppu {
    bus: Bus,
    frame: Vec<u8>,
}

#[allow(unused)]
struct Tile {}

impl Ppu {
    pub fn new(bus: Bus) -> Self {
        Ppu {
            bus,
            frame: Vec::new(),
        }
    }

    pub fn render(&self, lr: &mut LR35902) {
        // I just blindly increment the LY register for the lols
        //lr.set_mem8(0xFF44, lr.mem8(0xFF44).overflowing_add(1).0);
        lr.set_mem8(0xFF44, 144);
    }
}
