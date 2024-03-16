use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::core::components::tiles::atlas::data::{TileConfig, TilesetAtlas};

use crate::utils::file::read_file;

#[derive(Clone, Debug)]
/// Struct representing a tileset definition.
pub struct Tileset {
    pub(crate) name: String,
    /// Maximum number of tiles per line
    pub(crate) width: usize,
    /// Number of lines in the sprite
    pub(crate) height: usize,
    /// width of a tile
    pub(crate) tile_width: usize,
    /// height of a tile
    #[allow(dead_code)]
    pub(crate) tile_height: usize,
    /// Texture path of this Tileset
    pub(crate) texture: String,
    /// Pathing attached to the tileset
    pub(crate) pathing: HashMap<String, HashSet<usize>>,
    /// Configuration atteched to each tile
    pub(crate) tiles: HashMap<usize, TileConfig>,
}

impl Tileset {
    pub fn new(name: String, texture: String, width: usize,height: usize, tile_width: usize, tile_height: usize) -> Self {
        Self { name, width, height, tile_width,tile_height, texture, pathing: HashMap::default(), tiles: HashMap::default() }
    }

    pub fn with_pathing(mut self, pathing: HashMap<String, HashSet<usize>>) -> Self {
        self.pathing = pathing;
        self
    }

    pub fn from_atlas(path_to_atlas: &str, path_to_texture: &str) -> Result<Self, ()> {
        let path = Path::new(path_to_atlas);
        if path.exists() {
            match read_file(path) {
                Ok(bytes) => match serde_json::from_slice::<TilesetAtlas>(bytes.as_slice()) {
                    Ok(atlas) => {
                        return Ok(atlas.into_tileset(path_to_texture.to_string()));
                    }
                    Err(e) => {println!("{}", e)}
                },
                Err(e) => {println!("{:?}", e)}
            }
        }
        Err(())
    }
}
