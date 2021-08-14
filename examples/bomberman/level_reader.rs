use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Level {
    pub width: usize,
    pub height: usize,
    pub tilemap: Vec<LevelData>,
    pub character_x: usize,
    pub character_y: usize,
    pub pathing: Vec<Vec<usize>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelData {
    pub name: String,
    pub values: Vec<Vec<usize>>,
}

pub fn read_level(name: &str) -> Level {
    if let Ok(res) = scion::utils::file::read_file(Path::new(name)) {
        return serde_json::from_slice(res.as_slice()).expect("level error");
    }
    panic!("level error");
}
