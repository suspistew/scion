use std::path::Path;

use scion::Scion;
use scion::utils::file::app_base_path_join;

use crate::asset_package_config::StarlightPackage;
use crate::main_menu_scene::Menu;

mod main_menu_scene;
mod scripts;
mod level_scene;
mod asset_package_config;

fn main() {
    Scion::app_with_config_path(
        Path::new(&app_base_path_join("examples/starlight-1961/assets/main_config.json"))
    )
        .with_package(StarlightPackage)
        .with_scene::<Menu>()
        .run();
}