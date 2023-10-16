use crate::lr35902::LR35902;
use crate::memory::Bus;

//#[derive(Clone, PartialEq)]
pub struct Ppu {
    bus: Box<Bus>,
    frame: Vec<u8>,
}

impl Ppu {
    pub fn new(bus: Box<Bus>) -> Self {
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
