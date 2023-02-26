use std::{collections::HashMap, marker::PhantomData};

use crate::core::components::{material::Material, tiles::tileset::Tileset};
use crate::core::components::ui::font::Font;

/// `AssetManager` is resource that will link assets to an asset ref to allow reusability of assets
#[derive(Default)]
pub struct AssetManager {
    materials: HashMap<usize, Material>,
    fonts: HashMap<usize, Font>,
}

impl AssetManager {
    pub(crate) fn get_material_for_ref(&self, asset_ref: &AssetRef<Material>) -> Material {
        self.materials
            .get(&asset_ref.0)
            .expect("An asset has been requested but does not exist")
            .clone()
    }

    pub(crate) fn get_font_for_ref(&self, asset_ref: &AssetRef<Font>) -> Font {
        self.fonts
            .get(&asset_ref.0)
            .expect("An asset has been requested but does not exist")
            .clone()
    }

    pub fn register_material(&mut self, material: Material) -> AssetRef<Material> {
        let next_ref = AssetRef(self.materials.keys().count(), PhantomData::default());
        self.materials.insert(next_ref.0, material);
        next_ref
    }

    pub fn register_tileset(&mut self, tileset: Tileset) -> AssetRef<Material> {
        let next_ref = AssetRef(self.materials.keys().count(), PhantomData::default());
        self.materials.insert(next_ref.0, Material::Tileset(tileset));
        next_ref
    }

    pub fn register_font(&mut self, font: Font) -> AssetRef<Font> {
        let next_ref = AssetRef(self.materials.keys().count(), PhantomData::default());
        self.fonts.insert(next_ref.0, font);
        next_ref
    }

    pub fn retrieve_tileset(&self, asset_ref: &AssetRef<Material>) -> Option<&Tileset> {
        match self.materials.get(&asset_ref.0) {
            None => None,
            Some(material) => match material {
                Material::Tileset(tileset) => Some(tileset),
                _ => None,
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct AssetRef<T: Send + Sync>(pub(crate) usize, pub(crate) PhantomData<T>);

#[cfg(test)]
mod tests {
    use crate::core::{
        components::{color::Color, material::Material, tiles::tileset::Tileset},
        resources::asset_manager::AssetManager,
    };

    #[test]
    fn register_material_test() {
        let mut manager = AssetManager::default();
        let asset_ref = manager.register_material(Material::Color(Color::new(1, 1, 1, 1.)));
        assert_eq!(0, asset_ref.0);
        assert_eq!(1, manager.materials.len());

        let asset_ref = manager.register_material(Material::Color(Color::new(2, 2, 2, 1.)));
        assert_eq!(1, asset_ref.0);
        assert_eq!(2, manager.materials.len());
    }

    #[test]
    fn register_tileset_test() {
        let mut manager = AssetManager::default();
        let asset_ref = manager.register_tileset(Tileset::new("test".to_string(), 1, 1, 1));
        assert_eq!(0, asset_ref.0);
        assert_eq!(1, manager.materials.len());
    }
}
