use crate::lr35902::LR35902;
#[derive(Clone, PartialEq, Copy)]
pub struct Ppu {}

impl Ppu {
    pub fn new() -> Self {
        Ppu {}
    }

    pub fn render(&self, lr: &mut LR35902) {
        // I just blindly increment the LY register for the lols
        lr.set_mem8(0xFF44, lr.mem8(0xFF44).overflowing_add(1).0);
    }
}
