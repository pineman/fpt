use std::fmt;

/// Holds a 8x8 tile image as it appears in VRAM
/// (2 bytes for each 8 pixel row)
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    pub bytes: [u8; 16],
}

impl Tile {
    #[allow(unused)]
    pub fn load(data: &[u8; 16]) -> Tile {
        Tile { bytes: *data }
    }

    pub fn get_pixel(&self, y: usize, x: usize) -> u8 {
        let low_bit = (self.bytes[2 * y] >> (7 - x)) & 1;
        let high_bit = (self.bytes[2 * y + 1] >> (7 - x)) & 1;

        (high_bit << 1) + low_bit
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                write!(f, "{}", self.get_pixel(i, j))?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

/// Represents the data that lives in VRAM:
/// 3 * 128 tile blocks and two 32x32 tile maps
pub struct VRamContents {
    /// Three blocks of 128 tiles shared by the BG/Win tiles and OBJ tiles
    pub tile_data: [Tile; 384],
    /// The first 32x32 tile map, accessed when either LCDC.3 or LCDC.6 are 0
    pub tile_map0: [u8; 1024],
    /// The second 32x32 tile map, accessed when either LCDC.3 or LCDC.6 are 1
    pub tile_map1: [u8; 1024],
}

impl Default for VRamContents {
    fn default() -> VRamContents {
        VRamContents {
            tile_map0: [0; 1024],
            tile_map1: [0; 1024],
            tile_data: [Tile { bytes: [0; 16] }; 384],
        }
    }
}

impl VRamContents {
    pub fn load(vram: &[u8]) -> VRamContents {
        let mut tilemap = VRamContents::default();

        for i in 0..384 {
            tilemap.tile_data[i]
                .bytes
                .clone_from_slice(&vram[(16 * i)..(16 * (i + 1))]);
        }

        tilemap.tile_map0.clone_from_slice(&vram[0x1800..0x1c00]);
        tilemap.tile_map1.clone_from_slice(&vram[0x1c00..0x2000]);

        tilemap
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::map::VRAM;
    use crate::Gameboy;

    // Looks like a game boy
    #[rustfmt::skip]
    const THE_TILE: [u8; 16] = [
        0x3c, 0x7e,
        0x42, 0x42,
        0x42, 0x42,
        0x42, 0x42,
        0x7e, 0x5e,
        0x7e, 0x0a,
        0x7c, 0x56,
        0x38, 0x7c,
    ];

    #[test]
    #[rustfmt::skip]
    fn test_pixel_render() {
        let tile = Tile::load(&THE_TILE);

        let formatted = format!("{:?}", tile);

        assert_eq!(
            formatted,
            ["02333320",
                "03000030",
                "03000030",
                "03000030",
                "03133330",
                "01113130",
                "03131320",
                "02333200\n"].join("\n")
        )
    }

    #[test]
    fn test_one_tile_to_vram() {
        let gb: Gameboy = Gameboy::new();
        gb.bus
            .memory()
            .slice_mut(VRAM.start..VRAM.start + 16)
            .clone_from_slice(&THE_TILE[..]);

        // Parse the VRAM with our structs
        let tm: VRamContents = VRamContents::load(gb.bus.memory().slice(VRAM));

        assert_eq!(
            tm.tile_data[tm.tile_map0[0] as usize],
            Tile::load(&THE_TILE)
        );
    }

    #[test]
    fn test_photograph_ppu_frame_rendering_progress() {
        let mut gb: Gameboy = Gameboy::new();
        gb.bus
            .memory()
            .slice_mut(VRAM.start..VRAM.start + 16)
            .clone_from_slice(&THE_TILE[..]);

        std::fs::create_dir_all("screenshots").unwrap();
        for ly in 0..154 {
            crate::debugger::utilities::write_pgm_screenshot(
                gb.get_frame(),
                &format!("screenshots/test_one_tile_to_vram-ly_{ly:05}.pgm"),
            )
            .unwrap();
            gb.ppu.step(456);
        }
    }
}
