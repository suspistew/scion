use scion::core::package::Package;
use scion::core::resources::asset_manager::AssetType;
use scion::core::world::GameData;
use scion::ScionBuilder;
use scion::utils::file::app_base_path_join;
use crate::main_menu_scene::ACTIVE_MENU_FLAG;
use crate::scripts::background_parallax::apply_parallax_system;

#[derive(Default)]
pub struct StarlightPackage;
impl Package for StarlightPackage {
    fn prepare(&self, data: &mut GameData) {
        data.assets_mut().register_tileset_atlas_and_texture("main",
                                                             &app_base_path_join("examples/starlight-1961/assets/tileset_main.scion"),
                                                             &app_base_path_join("examples/starlight-1961/assets/main.png"));
        data.assets_mut().register_tileset_atlas_and_texture("space_ship",
                                                             &app_base_path_join("examples/starlight-1961/assets/space_ship.scion"),
                                                             &app_base_path_join("examples/starlight-1961/assets/space_ship.png"));
        data.assets_mut().register_atlas_path(AssetType::Tilemap("lvl1".to_string()), &app_base_path_join("examples/starlight-1961/assets/lvl1.scion"));
    }

    fn load(&self, builder: ScionBuilder) -> ScionBuilder {
        builder.with_pausable_system(apply_parallax_system, |gs| !gs.get_bool(ACTIVE_MENU_FLAG))
    }
}