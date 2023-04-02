use std::collections::HashMap;
use std::path::Path;
use log::{debug, error, info};

use scion::core::components::tiles::tilemap::TileEvent;
use scion::utils::maths::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    pub width: usize,
    pub height: usize,
    #[serde(default)]
    pub properties: HashMap<String, String>,
    #[serde(default)]
    pub layers: Vec<Layer>,
    pub objects: Vec<Object>,
}

impl Level {
    pub fn tile_at(&self, pos: &Position) -> isize {
        *self.layers.get(pos.z()).unwrap().tiles.get(pos.y()).unwrap().get(pos.x()).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    #[serde(default)]
    pub tiles: Vec<Vec<isize>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub name: String,
    pub x: usize,
    pub y: usize,
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

pub fn read_level(name: &str) -> Level {
    match scion::utils::file::read_file(Path::new(name)) {
        Ok(file) => {
            return serde_json::from_slice(file.as_slice()).expect("level error");
        }
        Err(e) => {
            std::panic::panic_any(e);
        }
    }
}
