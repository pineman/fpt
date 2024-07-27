use crate::bw::test_bit8;

#[derive(Debug)]
struct Flags {
    priority: bool,
    y_flip: bool,
    x_flip: bool,
    dmg_palette: bool,
    bank: bool,
    cgb_palette: bool,
}

impl Flags {
    pub fn from(memory: u8) -> Flags {
        Flags {
            priority: test_bit8::<0>(memory),
            x_flip: test_bit8::<1>(memory),
            y_flip: test_bit8::<2>(memory),
            dmg_palette: test_bit8::<3>(memory),
            bank: test_bit8::<4>(memory),
            cgb_palette: test_bit8::<5>(memory),
        }
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub y: u8,
    pub x: u8,
    pub tile_index: u8,
    flags: Flags,
}

impl Sprite {
    pub fn load(memory: &[u8]) -> Sprite {
        Sprite {
            y: memory[0],
            x: memory[1],
            tile_index: memory[2],
            flags: Flags::from(memory[3]),
        }
    }
}
