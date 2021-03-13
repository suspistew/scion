use scion::application::Scion;
use scion::legion::{system, Resources, World};
use scion::utils::time::Time;
use log::info;
use scion::utils::window::WindowDimensions;


use scion::renderer::{RendererType, ScionRenderer};
use miniquad::Context;
use scion::game_layer::{SimpleGameLayer, GameLayer, GameLayerType};
use scion::legion::systems::ParallelRunnable;
use scion::renderer::bidimensional::triangle::Triangle;
use scion::renderer::bidimensional::material::Material2D;
use scion::renderer::color::Color;


#[system(for_each)]
fn color(#[state] timer: &mut f32, #[resource] time: &Time, material: &mut Material2D) {
    *timer += time.delta_duration().as_secs_f32();
    if *timer > 0.01 {
        *timer = 0.;
        match material { Material2D::Color(color) => {
            let new_red = if color.red() < 255 { color.red() + 1 } else{ 0 };
            color.replace(Color::new_rgb(new_red, color.green(), color.blue()));
        } }
    }
}

#[derive(Default)]
struct Layer;

impl SimpleGameLayer for Layer {
    fn on_start(&mut self, world: &mut World, _resource: &mut Resources) {
        let components =
            (Triangle, Material2D::Color(Color::new(0, 47, 110, 1.0)));
        world.push(components);
    }
}

fn main() {
    Scion::app()
        .with_system(color_system(0.))
        .with_game_layer(GameLayer::weak::<Layer>())
        .run();
}