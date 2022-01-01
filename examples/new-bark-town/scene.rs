use legion::{Entity, EntityStore};
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
use scion::utils::maths::{Dimensions, Position};
use crate::level_reader::read_level;

#[derive(Default)]
pub struct MainScene{
    tilemap: Option<Entity>
}

impl Scene for MainScene {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        world.add_default_camera();
        resources.audio().play(new_bark_town_theme(), PlayConfig {
            volume: 0.2,
            looped: true,
            category: None,
        });
        let tilemap = load_map("examples/new-bark-town/assets/scenes/main_house_bedroom.json".to_string(), world, resources);
        self.tilemap = Some(tilemap);
    }

    fn on_update(&mut self, world: &mut World, resources: &mut Resources) {
        let ar = self.tilemap.as_ref().unwrap();
        let mut tilemap_entry = world.entry_mut(*ar).unwrap();
        let tilemap = tilemap_entry.get_component_mut::<Tilemap>().unwrap();

        let event = tilemap.retrieve_event(&Position::new(7, 0, 0)).unwrap();
        let mut props = event.properties();
        props.insert("SALOPRIE".to_string(), "valeur".to_string());
    }
}

fn load_map(level: String, world: &mut World, resources: &mut Resources) -> Entity {
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
    })
}


fn new_bark_town_theme() -> String {
    app_base_path().join("examples/new-bark-town/assets/newbarktown.ogg").get()
}