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
            Square,
        },
        game_layer::{GameLayer, SimpleGameLayer},
        resources::inputs::{inputs_controller::InputsController, keycode::KeyCode, InputState},
    },
    Scion,
};

#[derive(Default)]
struct BombermanLayer {
    entity: Option<Entity>,
}

impl SimpleGameLayer for BombermanLayer {
    fn on_start(&mut self, world: &mut World, _resources: &mut Resources) {
        let square = Square::new(100., None);
        let material = Material::Color(Color::new_rgb(255, 0, 0));
        let transform = Transform::new(Coordinates::new(100., 100.), 1., 0.);

        let move_right_anim = Animation::new(
            Duration::from_secs(1),
            vec![AnimationModifier::transform(
                120,
                Some(Coordinates::new(300., 0.)),
                Some(2.),
                None,
            )],
        );

        let mut animations_map = HashMap::new();
        animations_map.insert("MoveRight".to_string(), move_right_anim);
        let animations = Animations::new(animations_map);
        self.entity = Some(world.push((square, material, transform, animations)));
        world.push((Camera::new(768., 768., 10.), Transform::default()));
    }

    fn update(&mut self, world: &mut World, resources: &mut Resources) {
        let input_controller = resources.get::<InputsController>().unwrap();
        if input_controller
            .keyboard()
            .keyboard_events()
            .iter()
            .filter(|e| e.state.eq(&InputState::Pressed) || e.keycode.eq(&KeyCode::Right))
            .count()
            > 0
        {
            let mut entity = world.entry_mut(self.entity.unwrap()).unwrap();
            let animations = entity.get_component_mut::<Animations>().unwrap();
            let result = animations.run_once("MoveRight".to_string());
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
