use crate::bitwise;
use crate::memory::Bus;
use tile::TileMap;

mod tile;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
pub type Frame = [u8; WIDTH * HEIGHT];

//#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Ppu {
    bus: Bus,
    frame: Frame,
    dots_this_frame: u32,
    counter: u32,
    mode: Mode,
    tilemap: TileMap,
}

#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Mode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    PixelTransfer = 3,
}

impl Ppu {
    pub fn new(bus: Bus) -> Self {
        Ppu {
            bus,
            frame: [0b00; WIDTH * HEIGHT],
            dots_this_frame: 0,
            counter: 0,
            mode: Mode::OamScan,
            tilemap: TileMap::default(),
        }
    }

    pub fn step(&mut self, cycles: u8) {
        for _ in 0..cycles {
            self.dot();
        }
    }

    fn oam_scan(&mut self) {
        if self.dots_this_frame % 456 == 80 {
            self.tilemap = TileMap::load(&self.bus.vram());
            self.mode = dbg!(Mode::PixelTransfer);
        }
    }

    fn pixel_transfer(&mut self) {
        if self.dots_this_frame % 456 == 239 {
            self.mode = dbg!(Mode::HBlank);
            return;
        }

        let current_pixel = ((self.dots_this_frame % 456) - 80) as usize; // TODO I'm pretending the PPU never stalls
        let address = WIDTH * self.bus.ly() as usize + current_pixel;
        if address >= WIDTH * HEIGHT {
            self.bus.ly();
            self.dots_this_frame;
        }

        let column = address % WIDTH;
        let line = address / WIDTH;

        let tile = line + column / 32;

        let tile_data_address = self.tilemap.tile_map0[tile];

        let tile_x = column % 8;
        let tile_y = line % 8;

        let tile_pixel_address = 8 * tile_y + tile_x;
        self.frame[address] = (self.tilemap.tiles[tile].pixels[(tile_data_address >> 2) as usize]
            >> (tile_data_address & 0b11))
            & 0b11;
    }

    fn h_blank(&mut self) {
        if self.bus.ly() >= HEIGHT as u8 {
            self.mode = dbg!(Mode::VBlank);
            return;
        } else if self.dots_this_frame % 456 == 0 {
            self.mode = dbg!(Mode::OamScan);
        }
    }

    fn v_blank(&mut self) {
        if self.dots_this_frame == 0 {
            self.mode = dbg!(Mode::OamScan);
            return;
        }
    }

    fn state_machine(&mut self) {
        match self.mode {
            Mode::OamScan => self.oam_scan(),
            Mode::PixelTransfer => self.pixel_transfer(),
            Mode::HBlank => self.h_blank(),
            Mode::VBlank => self.v_blank(),
        };
    }

    fn dot(&mut self) {
        //! Simulates a "dot", as described in https://gbdev.io/pandocs/Rendering.html.
        //! A "dot" either draws a single pixel (in Mode 3) or is stalled for $REASONS.
        //! A "dot" = one 2^22 Hz time unit, so there's 4 dots per (DMG, single-speed) CPU cycle

        // Update LY register
        self.bus.set_ly((self.dots_this_frame / 456) as u8);

        // The timing of a frame consists of
        //   * 144 actual scanlines lasting 456 dots each, where:
        //     - the first 80 dots are mode 2 (OAM scan)
        //     - the next 172 to 289 dots are mode 3 (drawing pixels)
        //     - the remaining 87 to 204 dots are mode 0 (H-blank)
        //   * 10 "scanlines" (4560 dots) for mode 1 (V-blank)
        //let ppu_mode = if self.bus.ly() < HEIGHT as u8 {
        //    match self.dots_this_frame % 456 {
        //        0..80 => Mode::OamScan,         // Mode 2
        //        80..240 => Mode::PixelTransfer, // Mode 3 (TODO lasts between 172 and 289 dots)
        //        240.. => Mode::HBlank,          // Mode 0
        //    }
        //} else {
        //    Mode::VBlank // Mode 1
        //};

        self.state_machine();

        // Update "LYC == LY" and "PPU mode" flags in STAT register
        self.bus.set_stat(
            self.bus.stat() & 0b11111000
                | ((self.bus.ly() == self.bus.lyc()) as u8) << 2
                | self.mode as u8,
        );

        // TODO actually draw some actual background, window and sprites

        // Advance one "dot"
        self.dots_this_frame = (self.dots_this_frame + 1) % 70224;
        if self.dots_this_frame == 0 {
            self.counter += 1;
        }
    }

    pub fn get_frame(&self) -> &Frame {
        &self.frame
    }
}
