use std::fmt::{Display, Formatter};

use tile::VRamContents;

use crate::memory::Bus;

pub mod tile;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;
pub type Frame = [u8; WIDTH * HEIGHT]; // TODO: wasteful, each pixel is 2 bits only

//#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Ppu {
    bus: Bus,
    frame: Frame,
    dots_this_frame: u32,
    frame_counter: u32,
    mode: Mode,
    tilemap: VRamContents,
}

#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    PixelTransfer = 3,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mode {} ({:#?})", *self as u8, self)
    }
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0 => Mode::HBlank,
            1 => Mode::VBlank,
            2 => Mode::OamScan,
            3 => Mode::PixelTransfer,
            n => panic!("Tried to convert {n} to a ppu::Mode (valid values are 0, 1, 2 and 3)"),
        }
    }
}

pub const DOTS_IN_ONE_FRAME: u32 = 70224;

impl Ppu {
    pub fn new(mut bus: Bus) -> Self {
        // Make STAT's MODE bits consistent with the PPU's initial mode
        bus.set_stat(bus.stat() & 0b11111100 | Mode::OamScan as u8);

        Ppu {
            bus,
            frame: [0b00; WIDTH * HEIGHT],
            dots_this_frame: 0,
            frame_counter: 0,
            mode: Mode::OamScan,
            tilemap: VRamContents::default(),
        }
    }

    pub fn step(&mut self, cycles: u32) {
        for _ in 0..cycles {
            self.dot();
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn oam_scan(&mut self) {
        if self.dots_this_frame % 456 == (80 - 1) {
            self.tilemap = self.bus.with_vram(VRamContents::load);
            self.set_mode(Mode::PixelTransfer);
        }
    }

    /// Currently only draws the background pixels, not the window or sprites
    #[allow(clippy::format_collect)]
    fn pixel_transfer(&mut self) {
        if self.dots_this_frame % 456 == (80 + 160) as u32 {
            self.set_mode(Mode::HBlank);
            return;
        }
        // TODO: LCDC.3
        let x = ((self.dots_this_frame % 456) - 80) as usize; // TODO I'm pretending the PPU never stalls
        let y = self.bus.ly() as usize;
        let xx = ((x as u8 + self.bus.scx()) as u16 % 256u16) as usize;
        let yy = ((self.bus.ly() + self.bus.scy()) as u16 % 256u16) as usize;
        // TODO: LCDC.4
        let tile_i = xx / 8 + yy / 8 * 32;
        let tile_data_address = self.tilemap.tile_map0[tile_i];
        let tile = self.tilemap.tile_data[tile_data_address as usize];
        let pixel = tile.get_pixel(yy % 8, xx % 8);
        self.frame[WIDTH * y + x] = pixel;
    }

    fn h_blank(&mut self) {
        if self.dots_this_frame >= (456 * HEIGHT - 1) as u32 {
            self.set_mode(Mode::VBlank);
        } else if self.dots_this_frame % 456 == 455 {
            self.set_mode(Mode::OamScan);
        }
    }

    fn v_blank(&mut self) {
        if self.dots_this_frame == DOTS_IN_ONE_FRAME - 1 {
            self.set_mode(Mode::OamScan);
        }
    }

    /// Simulates a "dot", as described in https://gbdev.io/pandocs/Rendering.html.
    /// A "dot" either draws a single pixel (in Mode 3) or is stalled for $REASONS.
    /// A "dot" = one 2^22 Hz time unit, so there's 4 dots per machine cycle,
    /// or 1 dot each t-cycle. dot timings don't change on double speed mode.
    fn dot(&mut self) {
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

        match self.mode {
            Mode::OamScan => self.oam_scan(),
            Mode::PixelTransfer => self.pixel_transfer(),
            Mode::HBlank => self.h_blank(),
            Mode::VBlank => self.v_blank(),
        };

        // Update "LYC == LY" and "PPU mode" flags in STAT register
        self.bus.set_stat(
            self.bus.stat() & 0b11111000
                | ((self.bus.ly() == self.bus.lyc()) as u8) << 2
                | self.mode as u8,
        );

        // TODO actually draw some actual background, window and sprites

        // Advance one "dot"
        self.dots_this_frame = (self.dots_this_frame + 1) % DOTS_IN_ONE_FRAME;
        if self.dots_this_frame == 0 {
            self.frame_counter += 1;
        }
    }

    pub fn get_frame(&self) -> &Frame {
        &self.frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Gameboy;

    #[test]
    fn test_ppu_modes() {
        let mut gb: Gameboy = Gameboy::unsafely_optimized_new();
        assert_eq!(gb.ppu.mode, Mode::OamScan);
        gb.ppu.step(80);
        assert_eq!(gb.ppu.mode, Mode::PixelTransfer);
        gb.ppu.step(300);
        assert_eq!(gb.ppu.mode, Mode::HBlank);
        gb.ppu.step(76);
        assert_eq!(gb.ppu.mode, Mode::OamScan);
        gb.ppu.step(65208);
        assert_eq!(gb.ppu.mode, Mode::VBlank);
        gb.ppu.step(4560);
        assert_eq!(gb.ppu.mode, Mode::OamScan);
    }
}
