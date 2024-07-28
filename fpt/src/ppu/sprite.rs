//#[derive(Debug)]
//pub struct Flags {
//    pub priority: bool,
//    pub y_flip: bool,
//    pub x_flip: bool,
//    pub dmg_palette: u8,
//    pub bank: bool,
//    pub cgb_palette: u8,
//}

//impl Flags {
//    pub fn from(memory: u8) -> Flags {
//        Flags {
//            //priority: test_bit8::<7>(memory),
//            x_flip: test_bit8::<6>(memory),
//            y_flip: test_bit8::<5>(memory),
//            //dmg_palette: (memory >> 4) & 0b11,
//            //bank: test_bit8::<3>(memory),
//            //cgb_palette: memory & 0b111,
//        }
//    }
//}

#[derive(Debug)]
pub struct Sprite {
    pub y: u8,
    pub x: u8,
    pub tile_index: u8,
    //pub flags: Flags,
}

impl Sprite {
    pub fn load(memory: &[u8]) -> Sprite {
        Sprite {
            y: memory[0],
            x: memory[1],
            tile_index: memory[2],
            //flags: Flags::from(memory[3]),
        }
    }
}
