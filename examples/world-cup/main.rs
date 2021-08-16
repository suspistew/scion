use std::time::Duration;

use scion::{
    config::{scion_config::ScionConfigBuilder, window_config::WindowConfigBuilder},
    core::{
        components::{
            animations::{Animation, AnimationModifier, Animations},
            color::Color,
            material::Material,
            maths::{hierarchy::Parent, transform::Transform},
            Hide, Square,
        },
        game_layer::{GameLayer, SimpleGameLayer},
        legion_ext::{ScionResourcesExtension, ScionWorldExtension},
        resources::{
            events::PollConfiguration,
            inputs::keycode::KeyCode,
            sound::{Sound, SoundLoadingType},
        },
    },
    legion::{Entity, EntityStore, Resources, World},
    utils::file::app_base_path,
    Scion,
};

#[derive(Default)]
pub struct WorldCup {
    entity: Option<Entity>,
}

impl SimpleGameLayer for WorldCup {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        let animation = Animation::new(
            Duration::from_millis(500),
            vec![AnimationModifier::blink(1)],
        );

        let animations = Animations::single("color", animation);

        self.entity = Some(world.push((
            Square::new(500., None),
            Transform::from_xy(100., 100.),
            Material::Color(Color::new(0, 0, 255, 1.0)),
            animations,
        )));

        world.push((
            Square::new(500., None),
            Transform::from_xy(300., 100.),
            Material::Color(Color::new(0, 0, 255, 1.0)),
            Parent(self.entity.as_ref().unwrap().clone()),
        ));

        world.add_default_camera();
        resources.audio().register_sound(
            "test",
            Sound::new(
                app_base_path().join("examples/world-cup/assets/test.ogg").get(),
                SoundLoadingType::KeepAfterUse,
            ),
        );
        resources.audio().play("test", Default::default());
    }

    fn update(&mut self, world: &mut World, resources: &mut Resources) {
        let mut entry = world.entry_mut(*self.entity.as_ref().unwrap()).unwrap();
        let animations = entry.get_component_mut::<Animations>().unwrap();
        let mut play_sound = false;
        let mut stop_sound = false;
        resources.inputs().keyboard_mut().on_key_pressed(KeyCode::P, || {
            if animations.any_animation_running() {
                animations.stop_animation("color", false);
                stop_sound = true;
            } else {
                animations.loop_animation("color");
                play_sound = true;
            }
        });

        if play_sound {
            resources.audio().play("test", Default::default());
        }
        if stop_sound {
            resources.audio().stop("test");
        }
    }
}

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(
                WindowConfigBuilder::new()
                    .with_default_background_color(Some(Color::new_rgb(0, 0, 0)))
                    .get(),
            )
            .get(),
    )
    .with_game_layer(GameLayer::strong::<WorldCup>("main layer"))
    .run();
}
