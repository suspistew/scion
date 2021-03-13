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

#[system]
fn time(#[resource] time: &Time) {
    info!("Time elapsed in the last frame : {:?}", time.delta_duration())
}

#[derive(Default)]
struct Layer;
impl SimpleGameLayer for Layer {
    fn on_start(&mut self, world: &mut World, _resource: &mut Resources) {
        world.push((Triangle,));
    }

    fn update(&mut self, _world: &mut World, _resource: &mut Resources) {
        info!("Hello from game Layer");
    }
}

fn main() {
    Scion::app()
        .with_system(time_system())
        .with_game_layer(GameLayer::weak::<Layer>())
        .run();
}