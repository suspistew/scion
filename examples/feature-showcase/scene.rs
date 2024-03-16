use scion::core::components::tiles::atlas::importer::load_tilemap;
use scion::core::resources::asset_manager::AssetType;

use scion::core::scene::Scene;
use scion::core::world::{GameData, World};
use scion::utils::file::app_base_path_join;

#[derive(Default)]
pub struct DemoScene;

impl Scene for DemoScene{
    fn on_start(&mut self, data: &mut GameData) {
        data.add_default_camera();

        data.assets_mut().register_tileset_atlas_and_texture(
            "demo-ply", &app_base_path_join("examples/feature-showcase/assets/demo-ply.scion"),
            &app_base_path_join("examples/feature-showcase/assets/nbt.png")
        );

        data.assets_mut().register_atlas_path(AssetType::Tilemap("test-lvl".to_string()), &app_base_path_join("examples/feature-showcase/assets/test-lvl.scion"));
        load_tilemap(data, "test-lvl");
    }
}