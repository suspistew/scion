use std::collections::HashMap;
use ab_glyph::FontVec;
use crate::core::components::color::Color;
use crate::core::components::material::Texture;
use crate::core::components::ui::font::Font;
use crate::core::resources::asset_manager::AssetRef;
use crate::utils::ScionError;

#[derive(Default)]
pub(crate) struct FontAtlas {
    atlas: HashMap<String, HashMap<usize, HashMap<Color, Texture>>>,
}

impl FontAtlas {
    pub fn get_texture(&self, font: &str, font_size: usize, font_color: &Color) -> Option<&Texture> {
        if self.atlas.contains_key(font) {
            let size_map = self.atlas.get(font).unwrap();
            if size_map.contains_key(&font_size) {
                let color_map = size_map.get(&font_size).unwrap();
                if color_map.contains_key(font_color) {
                    return color_map.get(font_color);
                }
            }
        }
        None
    }

    pub fn add(&mut self, font: String, font_size: usize, font_color: Color, texture: Texture) {
        let mut size_map = self.atlas.entry(font).or_insert_with(|| HashMap::default());
        let mut color_map = size_map.entry(font_size).or_insert_with(|| HashMap::default());
        color_map.insert(font_color, texture);
    }

    pub fn generate_bitmap(font: Font, font_size: usize, font_color: &Color) -> Result<(), ScionError>{
        if let Font::TrueType { font_path} = font {
            //let font = FontVec::try_from_vec();
        }
        Err(ScionError::new("Wrong type sent to bitmap generation"))
    }
}