use std::{collections::HashMap, time::Duration};

use legion::{Entity, EntityStore, Resources, World};
use scion::{
    config::{scion_config::ScionConfigBuilder, window_config::WindowConfigBuilder},
    core::{
        components::{
            animations::{Animation, AnimationModifier, Animations},
            color::Color,
            material::Material,
            maths::{
                camera::Camera,
                transform::{Coordinates, Transform},
            },
            tiles::{sprite::Sprite, tileset::Tileset},
            Square,
        },
        game_layer::{GameLayer, SimpleGameLayer},
        resources::{
            asset_manager::AssetManager,
            inputs::{inputs_controller::InputsController, keycode::KeyCode, InputState},
        },
    },
    utils::file::app_base_path,
    Scion,
};

#[derive(Default)]
struct BombermanLayer {
    entity: Option<Entity>,
}

impl SimpleGameLayer for BombermanLayer {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        let asset_ref = resources
            .get_mut::<AssetManager>()
            .expect("Asset Manager is mandatory")
            .register_tileset(Tileset::new(
                app_base_path()
                    .join("examples/bomberman/assets/sokoban_tilesheet.png")
                    .get(),
                13,
                8,
                64,
            ));
        let sprite = Sprite::new(78);
        let transform = Transform::new(Coordinates::new(100., 100.), 1., 0.);

        let move_right_anim = Animation::new(
            Duration::from_millis(500),
            vec![
                AnimationModifier::transform(120, Some(Coordinates::new(64., 0.)), None, None),
                AnimationModifier::sprite(vec![79, 78, 80, 78, 79], 78),
            ],
        );

        let mut animations_map = HashMap::new();
        animations_map.insert("MoveRight".to_string(), move_right_anim);
        let animations = Animations::new(animations_map);
        self.entity = Some(world.push((sprite, asset_ref, transform, animations)));
        world.push((Camera::new(768., 768., 10.), Transform::default()));
    }

    fn update(&mut self, world: &mut World, resources: &mut Resources) {
        let input_controller = resources.get::<InputsController>().unwrap();
        if input_controller.keyboard().key_pressed(&KeyCode::Right) {
            let mut entity = world.entry_mut(self.entity.unwrap()).unwrap();
            let animations = entity.get_component_mut::<Animations>().unwrap();
            let _result = animations.loop_animation("MoveRight".to_string());
        } else {
            let mut entity = world.entry_mut(self.entity.unwrap()).unwrap();
            let animations = entity.get_component_mut::<Animations>().unwrap();
            let _result = animations.stop_animation("MoveRight".to_string(), false);
        }
    }
}

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(
                WindowConfigBuilder::new()
                    .with_resizable(false)
                    .with_dimensions((768, 768))
                    .with_default_background_color(Some(Color::new_rgb(150, 100, 0)))
                    .get(),
            )
            .get(),
    )
    .with_game_layer(GameLayer::strong::<BombermanLayer>("Bomberman"))
    .run();
}
