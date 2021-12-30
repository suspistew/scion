use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use scion::core::components::tiles::tilemap::TileEvent;
use scion::utils::file::FileReaderError;
use scion::utils::maths::Position;

impl ScionMap{
    pub fn tile_at(&self, pos: &Position) -> usize {
        *self.layers
            .get(pos.z())
            .unwrap()
            .tiles
            .get(pos.y())
            .unwrap()
            .get(pos.x())
            .unwrap()
    }

    pub fn event_at(&mut self, pos: &Position) -> Option<TileEvent> {
        if let Some(layer) = self.layers
            .get_mut(pos.z()) {
            if let Some(event) = layer.events.iter().filter(|event| event.x == pos.x() && event.y == pos.y()).next(){
                let mut event_type = String::default();
                let mut properties = HashMap::new();
                for item in event.properties.iter() {
                    if item.name.as_str().eq("type") {
                        event_type = item.value.to_string();
                    }else{
                        properties.insert(item.name.to_string(), item.value.to_string());
                    }
                }
                return Some(TileEvent::new(event_type, properties));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScionMap{
    pub width: usize,
    pub height: usize,
    pub properties: Vec<Property>,
    pub layers: Vec<ScionLayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScionLayer {
    pub name: String,
    pub tiles: Vec<Vec<usize>>,
    pub events: Vec<Event>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub x: usize,
    pub y: usize,
    pub properties: Vec<Property>,
}


pub fn read_level(name: &str) -> ScionMap {
    println!("name : {:?}", name);
    match scion::utils::file::read_file(Path::new(name)) {
        Ok(file) => {
            return serde_json::from_slice(file.as_slice()).expect("level error");
        }
        Err(e) => {
            panic!(e);
        }
    }
}