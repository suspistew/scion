use std::collections::HashMap;
use std::path::Path;

use scion::graphics::components::tiles::tilemap::TileEvent;

use scion::utils::maths::Position;
use serde::{Deserialize, Serialize};

impl ScionMap {
    pub fn tile_at(&self, pos: &Position) -> usize {
        *self.layers.get(pos.z()).unwrap().tiles.get(pos.y()).unwrap().get(pos.x()).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScionLevel {
    pub map: ScionMap,
    pub events: Vec<ScionEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScionEvent {
    pub event_type: String,
    pub x: usize,
    pub y: usize,
    pub properties: HashMap<String, String>,
}

impl ScionLevel {
    pub fn event_at(&mut self, pos: &Position) -> Option<TileEvent> {
        for event in self.events.iter() {
            if event.x == pos.x() && event.y == pos.y() {
                return Some(TileEvent::new(
                    event.event_type.to_string(),
                    event.properties.clone(),
                ));
            }
        }
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScionMap {
    pub width: usize,
    pub height: usize,
    #[serde(default)]
    pub properties: Vec<Property>,
    #[serde(default)]
    pub layers: Vec<ScionLayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScionLayer {
    pub name: String,
    #[serde(default)]
    pub tiles: Vec<Vec<usize>>,
    #[serde(default)]
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub x: usize,
    pub y: usize,
    #[serde(default)]
    pub properties: Vec<Property>,
}

pub fn read_level(name: &str) -> ScionLevel {
    match scion::utils::file::read_file(Path::new(name)) {
        Ok(file) => {
            return serde_json::from_slice(file.as_slice()).expect("level error");
        }
        Err(e) => {
            std::panic::panic_any(e);
        }
    }
}
