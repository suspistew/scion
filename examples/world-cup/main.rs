use scion::Scion;
use scion::core::game_layer::{GameLayer, SimpleGameLayer};
use scion::legion::{World, Resources, Entity, EntityStore};

use scion::core::components::material::Material;

use scion::core::components::maths::camera::Camera;
use scion::core::components::maths::transform::{Transform};

use scion::core::components::Square;
use scion::core::components::animations::{Animation, AnimationModifier, Animations};
use std::time::Duration;
use scion::core::components::color::Color;

use scion::core::resources::inputs::keycode::KeyCode;
use scion::config::scion_config::{ScionConfigBuilder};
use scion::config::window_config::WindowConfigBuilder;
use scion::core::legion_ext::ScionExtension;

#[derive(Default)]
pub struct WorldCup{
    entity: Option<Entity>
}

impl SimpleGameLayer for WorldCup{
    fn on_start(&mut self, world: &mut World, _resources: &mut Resources) {

        let animation = Animation::new(Duration::from_millis(2000),
                                       vec![AnimationModifier::color(60, Color::new(125, 176, 0, 1.0 ))], false);

        let animations = Animations::single("color".to_string(), animation);

        self.entity = Some(world.push((Square::new(500., None), Transform::from_xy(100., 100.), Material::Color(Color::new(0, 0, 255, 1.0)), animations)));

        world.push((Camera::new(1024., 768., 10.), Transform::default()));

    }

    fn update(&mut self, world: &mut World, resources: &mut Resources) {
        let mut entry = world.entry_mut(*self.entity.as_ref().unwrap()).unwrap();
        let animations = entry.get_component_mut::<Animations>().unwrap();
        resources.inputs().keyboard_mut().on_key_pressed(KeyCode::P, || {
            animations.run_animation("color".to_string());
        })
    }
}

fn main() {
    Scion::app_with_config(ScionConfigBuilder::new().with_window_config(WindowConfigBuilder::new().with_default_background_color(Some(Color::new_rgb(0,0,0))).get()).get())
        .with_game_layer(GameLayer::strong::<WorldCup>("main layer"))
        .run();
}