use crate::memory::Bus;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

//#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Ppu {
    bus: Bus,
    frame: [u8; WIDTH * HEIGHT],
    dots_this_frame: u32,
}

#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
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
        }
    }

    pub fn step(&mut self, cycles:u8) {
        for _ in 0..cycles {
            self.dot();
        }
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
        let ppu_mode = if self.bus.ly() < HEIGHT as u8 {
            match self.dots_this_frame % 456 {
                0..80 => Mode::OamScan,         // Mode 2
                80..240 => Mode::PixelTransfer, // Mode 3 (TODO lasts between 172 and 289 dots)
                240.. => Mode::HBlank,           // Mode 0
            }
        } else {
            Mode::VBlank // Mode 1
        };

        // Update "LYC == LY" and "PPU mode" flags in STAT register
        self.bus.set_stat(
            self.bus.stat() & 0b11111000
                | ((self.bus.ly() == self.bus.lyc()) as u8) << 2
                | ppu_mode as u8,
        );

        // TODO actually draw some actual background, window and sprites
        if ppu_mode == Mode::PixelTransfer {
            let current_pixel = ((self.dots_this_frame % 456) - 80) as usize; // TODO I'm pretending the PPU never stalls
            let address = WIDTH * self.bus.ly() as usize + current_pixel;
            if address >= WIDTH * HEIGHT {
                dbg!(self.bus.ly());
                dbg!(self.dots_this_frame);
            }

            self.frame[address] = 0b00;
        }

        // Advance one "dot"
        self.dots_this_frame = (self.dots_this_frame + 1) % 70224;
    }

    pub fn get_frame(&self) -> &[u8; WIDTH * HEIGHT] {
        &self.frame
    }
}
