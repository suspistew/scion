use scion::game_layer::{GameLayer, GameLayerController, SimpleGameLayer};
use scion::legion::{Resources, system, World};
use scion::rendering::bidimensional::{Camera2D, Material2D, Position2D, Transform2D};
use scion::rendering::bidimensional::components::Square;
use scion::Scion;

#[system]
fn test() {
    log::info!("Hello all");
}

#[derive(Default)]
struct LayerA;

impl SimpleGameLayer for LayerA {
    fn on_start(&mut self, world: &mut World, resource: &mut Resources) {

    }
}

fn main() {
    Scion::app()
        .with_game_layer(
            GameLayer::weak::<LayerA>()
        )
        .run();
}