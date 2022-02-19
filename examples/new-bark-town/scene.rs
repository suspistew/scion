use std::fmt::format;
use std::time::Duration;

use legion::{Entity, EntityStore};

use scion::core::components::animations::{Animation, AnimationModifier, Animations};
use scion::core::components::color::Color;
use scion::core::components::material::Material;
use scion::core::components::maths::camera::Camera;
use scion::core::components::maths::hierarchy::Parent;
use scion::core::components::maths::transform::{Transform, TransformBuilder};
use scion::core::components::shapes::rectangle::Rectangle;
use scion::core::components::Square;
use scion::core::components::tiles::sprite::Sprite;
use scion::core::components::tiles::tilemap::{TileEvent, TileInfos, Tilemap, TilemapInfo};
use scion::core::components::tiles::tileset::Tileset;
use scion::core::components::ui::ui_image::UiImage;
use scion::core::legion_ext::{ScionResourcesExtension, ScionWorldExtension};
use scion::core::resources::asset_manager::AssetRef;
use scion::core::resources::audio::PlayConfig;
use scion::core::resources::inputs::types::KeyCode;
use scion::core::resources::time::TimerType;
use scion::core::scene::Scene;
use scion::legion::{Resources, World};
use scion::utils::file::app_base_path;
use scion::utils::maths::{Dimensions, Position};

use crate::animations;
use crate::animations::{FADE_DURATION, MOVE_DURATION, switch_scene_animation};
use crate::level_reader::read_level;

pub struct GlobalResource {
    level: String,
    start_x: usize,
    start_y: usize,
    direction: String,
}

impl Default for GlobalResource {
    fn default() -> Self {
        Self {
            level: "main_house_bedroom".to_string(),
            start_x: 1,
            start_y: 5,
            direction: "TOP".to_string(),
        }
    }
}

#[derive(Default, Debug)]
pub struct MainCharacter {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

#[derive(Default)]
pub struct MainScene {
    tilemap: Option<Entity>,
    player: Option<Entity>,
    fader: Option<Entity>,
    current_width: usize,
    current_height: usize,
    is_switching: bool
}

impl Scene for MainScene {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        if !resources.contains::<GlobalResource>() {
            resources.insert(GlobalResource::default());
            resources.timers().add_timer("SceneSwitch", TimerType::Manual, 0.5);
            play_music(resources);
        }
        self.fader = Some(world.push((
            Rectangle::new(384., 336., None),
            TransformBuilder::new().with_translation(0.,0.,10).with_screen_as_origin().build(),
            Material::Color(Color::new(255,255,255,1.)),
            Animations::new(switch_scene_animation())
        )));

        let (level, start_x, start_y, direction) = {
            let global_resource = resources.get::<GlobalResource>().unwrap();
            (format!("examples/new-bark-town/assets/scenes/{}.json", global_resource.level.to_string()), global_resource.start_x, global_resource.start_y, global_resource.direction.to_string())
        };


        let tilemap = self.load_map(level, world, resources);
        self.tilemap = Some(tilemap);
        let char = add_character(world, resources, start_x, start_y, &direction);
        let mut camera_transform = Transform::from_xy(-192., -168.);
        world.push((
            Camera::new(384., 336.),
            camera_transform,
            Parent(char),
        ));
        self.player = Some(char);
    }

    fn on_update(&mut self, world: &mut World, resources: &mut Resources) {
        if self.is_switching {
            let fader = world.entry(*self.fader.as_ref().unwrap()).unwrap();
            let anim = fader.get_component::<Animations>().unwrap();
            if !anim.any_animation_running(){
                world.clear();
                resources.scene_controller().switch::<MainScene>();
            }
            return;
        }

        if !resources.timers().get_timer("SceneSwitch").unwrap().ended(){
            return;
        }

        let (mut tilemap_world, mut player_world) = world.split::<&mut Tilemap>();


        let (pos_x, pos_y, delta_x, delta_y) = {
            let mut player = player_world.entry_mut(*self.player.as_ref().unwrap()).expect("Player is mandatory");
            let transform = player.get_component::<Transform>().expect("Transform is mandatory on player");
            (transform.translation().x() as usize / 48,
             transform.translation().y() as usize / 48,
             transform.translation().x() as usize % 48,
             transform.translation().y() as usize % 48
            )
        };

        let mut tilemap_entry = tilemap_world.entry_mut(*self.tilemap.as_ref().unwrap()).expect("Tilemap is mandatory");
        let mut tilemap = tilemap_entry.get_component_mut::<Tilemap>().expect("Tilemap is mandatory");

        let (left, right, top, bottom) = {
            (pos_x > 0 && tilemap.retrieve_pathing(&Position::new(pos_x - 1, pos_y, 0), &mut player_world, &resources.assets()).is_some(),
             pos_x < self.current_width - 1 && tilemap.retrieve_pathing(&Position::new(pos_x + 1, pos_y, 0), &mut player_world, &resources.assets()).is_some(),
             pos_y > 0 && tilemap.retrieve_pathing(&Position::new(pos_x, pos_y - 1, 0), &mut player_world, &resources.assets()).is_some(),
             pos_y < self.current_height - 1 && tilemap.retrieve_pathing(&Position::new(pos_x, pos_y + 1, 0), &mut player_world, &resources.assets()).is_some())
        };

        {
            let mut player = player_world.entry_mut(*self.player.as_ref().unwrap()).expect("Player is mandatory");
            let player = player.get_component_mut::<MainCharacter>().expect("Transform is mandatory on player");
            player.left = left;
            player.right = right;
            player.top = top;
            player.bottom = bottom;
        }

        let event = tilemap.retrieve_event(&Position::new(pos_x, pos_y, 0));
        if let Some(e) = event {
            if (e.event_type().eq("DOOR") && delta_x == 0 && delta_y == 0) || e.event_type().eq("EXIT") && resources.inputs().key_pressed(&KeyCode::Down) {
                let (level_name, target_x, target_y, direction) =
                    (e.properties().get("go_to").unwrap().to_string(),
                     e.properties().get("target_x").unwrap().parse::<usize>().unwrap(),
                     e.properties().get("target_y").unwrap().parse::<usize>().unwrap(),
                     e.properties().get("direction").unwrap().to_string());
                {
                    let mut level = resources.get_mut::<GlobalResource>().expect("Global resource is mandatory");
                    level.level = level_name;
                    level.start_x = target_x;
                    level.start_y = target_y;
                    level.direction = direction;
                }

                play_door_effect(resources);
                self.is_switching = true;

                let mut fader = player_world.entry_mut(*self.fader.as_ref().unwrap()).unwrap();
                let mut anim = fader.get_component_mut::<Animations>().unwrap();
                anim.run_animation("FADE_IN");

                resources.timers().get_timer("SceneSwitch").unwrap().reset();
            }
        }
    }
}

impl MainScene {
    fn load_map(&mut self, level: String, world: &mut World, resources: &mut Resources) -> Entity {
        let asset_ref = resources.assets().register_tileset(Tileset::from_atlas("examples/new-bark-town/assets/nbt_atlas.json").unwrap());
        let mut level = read_level(level.as_str());
        let mut scale = Transform::default();
        scale.set_scale(3.0);
        let tilemap_infos = TilemapInfo::new(
            Dimensions::new(level.map.width, level.map.height, level.map.layers.len()),
            scale,
            asset_ref.clone(),
        );

        self.current_width = level.map.width;
        self.current_height = level.map.height;
        Tilemap::create(tilemap_infos, world, |p| {
            TileInfos::new(Some(level.map.tile_at(p)), get_animation_for_tile(level.map.tile_at(p)))
                .with_event(level.event_at(p))
        })
    }
}

fn play_music(resources: &mut Resources) {
    resources.audio().play(new_bark_town_theme(), PlayConfig {
        volume: 0.2,
        looped: true,
        category: None,
    });
}

fn play_door_effect(resources: &mut Resources) {
    resources.audio().play(door_effect(), PlayConfig {
        volume: 0.2,
        looped: false,
        category: None,
    });
}

fn get_animation_for_tile(i: usize) -> Option<Animation> {
    if i == 73 {
        Some(Animation::looping(Duration::from_millis(1500), vec![AnimationModifier::sprite(vec![74, 73, 74, 73, 74], 73)]))
    } else if i == 85 {
        Some(Animation::looping(Duration::from_millis(1500), vec![AnimationModifier::sprite(vec![86, 85, 86, 85, 86], 85)]))
    } else if i == 96 {
        Some(Animation::looping(Duration::from_millis(2000), vec![AnimationModifier::sprite(vec![51, 75, 99, 96, 51], 96)]))
    } else if i == 108 {
        Some(Animation::looping(Duration::from_millis(2000), vec![AnimationModifier::sprite(vec![63, 87, 111, 108, 63], 108)]))
    } else if i == 97 {
        Some(Animation::looping(Duration::from_millis(2000), vec![AnimationModifier::sprite(vec![52, 76, 100, 97, 52], 97)]))
    } else if i == 109 {
        Some(Animation::looping(Duration::from_millis(2000), vec![AnimationModifier::sprite(vec![64, 88, 112, 109, 64], 109)]))
    } else {
        None
    }
}

fn add_character(world: &mut World, resources: &mut Resources, start_x: usize, start_y: usize, direction: &String) -> Entity {
    let asset_ref = resources.assets().register_tileset(Tileset::new("examples/new-bark-town/assets/character.png".to_string(), 10, 8, 16));
    let mut animations = Animations::new(animations::char_animations());
    if direction.eq("BOTTOM"){
        animations.run_animation("MOVE_BOTTOM");
    }
    world.push(
        (TransformBuilder::new().with_translation(start_x as f32 * 16. * 3., start_y as f32 * 16. * 3., 2).with_scale(3.).build(),
         Sprite::new(get_direction_sprite(direction)),
         asset_ref,
         animations,
         MainCharacter::default()
        )
    )
}

fn get_direction_sprite(direction: &String) -> usize {
    if direction.eq("BOTTOM") {
        1
    } else {
        4
    }
}


fn new_bark_town_theme() -> String {
    app_base_path().join("examples/new-bark-town/assets/newbarktown.ogg").get()
}

fn door_effect() -> String {
    app_base_path().join("examples/new-bark-town/assets/door.ogg").get()
}