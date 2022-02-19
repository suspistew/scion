use crate::utils::file::read_file;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::path::Path;

#[derive(Clone, Debug)]
/// Struct representing a tileset definition.
pub struct Tileset {
    /// Maximum number of tiles per line
    pub(crate) width: usize,
    /// Number of lines in the sprite
    pub(crate) height: usize,
    /// Size of a tile
    pub(crate) tile_size: usize,
    /// Texture path of this Tileset
    pub(crate) texture: String,
    /// Pathing attached to the tileset
    pub(crate) pathing: HashMap<String, HashSet<usize>>,
}

impl Tileset {
    pub fn new(texture: String, width: usize, height: usize, tile_size: usize) -> Self {
        Self { width, height, tile_size, texture, pathing: HashMap::default() }
    }

    pub fn with_pathing(mut self, pathing: HashMap<String, HashSet<usize>>) -> Self {
        self.pathing = pathing;
        self
    }

    pub fn from_atlas(path_to_atlas: &str) -> Result<Self, ()> {
        let path = Path::new(path_to_atlas);
        if path.exists() {
            match read_file(path) {
                Ok(bytes) => match serde_json::from_slice::<TilesetAtlas>(bytes.as_slice()) {
                    Ok(atlas) => {
                        let mut pathing = HashMap::new();
                        for item in atlas.pathing.iter() {
                            pathing.insert(item.pathing_type.to_string(), item.tiles.clone());
                        }
                        return Ok(Self {
                            width: atlas.width,
                            height: atlas.height,
                            tile_size: atlas.tile_size,
                            texture: atlas.texture,
                            pathing,
                        });
                    }
                    Err(_) => {}
                },
                Err(_) => {}
            }
        }
        return Err(());
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TilesetAtlas {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) tile_size: usize,
    pub(crate) texture: String,
    pub(crate) pathing: Vec<PathingValue>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PathingValue {
    pub(crate) pathing_type: String,
    pub(crate) tiles: HashSet<usize>,
}

impl FromIterator<PathingValue> for HashMap<String, HashSet<usize>> {
    fn from_iter<T: IntoIterator<Item = PathingValue>>(iter: T) -> Self {
        let mut c = HashMap::new();
        for item in iter {
            c.insert(item.pathing_type, item.tiles);
        }
        c
    }
}
