#[derive(Clone, Debug)]
pub struct Tileset {
    /// Maximum number of tiles per line
    pub(crate) length: usize,
    /// Number of lines in the sprite
    pub(crate) height: usize,
    /// Size of a tile
    pub(crate) tile_size: usize,
    /// Texture path of this Tileset
    pub(crate) texture: String,
}

impl Tileset {
    pub fn new(texture_path: String, length: usize, height: usize, tile_size: usize) -> Self {
        Self {
            length,
            height,
            tile_size,
            texture: texture_path,
        }
    }
}
