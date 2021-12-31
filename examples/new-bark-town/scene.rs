use scion::core::components::material::Material;
use scion::core::components::maths::transform::Transform;
use scion::core::components::tiles::tilemap::{TileEvent, TileInfos, Tilemap, TilemapInfo};
use scion::core::components::tiles::tileset::Tileset;
use scion::core::legion_ext::{ScionResourcesExtension, ScionWorldExtension};
use scion::core::resources::asset_manager::AssetRef;
use scion::core::resources::audio::PlayConfig;
use scion::core::scene::Scene;
use scion::legion::{Resources, World};
use scion::utils::file::app_base_path;
use scion::utils::maths::Dimensions;
use crate::level_reader::read_level;

#[derive(Default)]
pub struct MainScene{
    tileset_ref: Option<AssetRef<Material>>
}

impl Scene for MainScene {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        world.add_default_camera();
        resources.audio().play(new_bark_town_theme(), PlayConfig {
            volume: 0.2,
            looped: true,
            category: None,
        });
        let asset_ref = load_map("examples/new-bark-town/assets/scenes/main_house_bedroom.json".to_string(), world, resources);
        self.tileset_ref = Some(asset_ref);
    }

    fn on_update(&mut self, _world: &mut World, resources: &mut Resources) {
        let ar = self.tileset_ref.as_ref().unwrap();
        let assetManager = resources.assets();
        let tileset = assetManager.retrieve_tileset(ar);

        match tileset {
            None => {}
            Some(t) => { println!("{:?}", t);}
        }

    }
}

fn load_map(level: String, world: &mut World, resources: &mut Resources) -> AssetRef<Material> {
    let asset_ref = resources.assets().register_tileset(Tileset::from_atlas("examples/new-bark-town/assets/tileset_atlas.json").unwrap());
    let mut level = read_level(level.as_str());
    let mut scale = Transform::default();
    scale.set_scale(3.0);
    let tilemap_infos = TilemapInfo::new(
        Dimensions::new(level.width, level.height, level.layers.len()),
        scale,
        asset_ref.clone(),
    );

    Tilemap::create(tilemap_infos, world, |p| {
        TileInfos::new(Some(level.tile_at(p)), None)
            .with_event(level.event_at(p))
    });
    asset_ref
}


fn new_bark_town_theme() -> String {
    app_base_path().join("examples/new-bark-town/assets/newbarktown.ogg").get()
}