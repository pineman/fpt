
use std::fmt;

#[derive(Copy, Clone)]
pub struct Tile {
    pub pixels: [u8; 16],
}

impl Tile {

    pub fn load(data: &[u8; 16]) -> Tile {
        Tile {
            pixels: data.clone(),
        }
    }
    pub fn get_pixel(&self, y: usize, x: usize) -> u8{
        let low_bit = (self.pixels[2*y] >> (7-x)) & 1;
        let high_bit = (self.pixels[2*y + 1] >> (7-x)) & 1;

        (high_bit << 1) + low_bit
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                write!(f, "{}", self.get_pixel(i, j));
            }
            write!(f, "\n");

        }
        write!(f, "")
    }
}

pub struct TileMap {
    pub tile_map0: [u8; 1024],
    pub tile_map1: [u8; 1024],
    pub tiles: [Tile; 384],
}

impl TileMap {
    pub fn default() -> TileMap {
        TileMap {
            tile_map0: [0; 1024],
            tile_map1: [0; 1024],
            tiles: [Tile { pixels: [0; 16] }; 384],
        }
    }

    pub fn load(vram: &Vec<u8>) -> TileMap {
        let mut tilemap = TileMap::default();

        for i in 0..384 {
            tilemap.tiles[i]
                .pixels
                .clone_from_slice(&vram[(16 * i).. (16 * (i + 1))]);
        }

        tilemap.tile_map0.clone_from_slice(&vram[0x1800..0x1c00]);
        tilemap.tile_map1.clone_from_slice(&vram[0x1c00..0x2000]);

        tilemap
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_render() {
        let tile = Tile::load(&[0x3c, 0x7e, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7e, 0x5e, 0x7e, 0x0a, 0x7c, 0x56, 0x38, 0x7c]);

        let formatted = format!("{:?}", tile);

        assert_eq!(formatted, "02333320\n03000030\n03000030\n03000030\n03133330\n01113130\n03131320\n02333200\n")
    }
}
