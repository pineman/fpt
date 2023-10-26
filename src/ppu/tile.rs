#[derive(Copy, Clone)]
pub struct Tile {
    pub pixels: [u8; 16],
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
                .clone_from_slice(&vram[16 * i..16 * (i + 1)]);
        }

        tilemap.tile_map0.clone_from_slice(&vram[0x1800..0x1c00]);
        tilemap.tile_map1.clone_from_slice(&vram[0x1c00..0x2000]);

        tilemap
    }
}
