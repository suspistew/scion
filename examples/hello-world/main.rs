use std::path::Path;

use scion::application::Scion;
use scion::game_layer::{GameLayer, SimpleGameLayer};
use scion::legion::{Resources, World, system};

use scion::rendering::bidimensional::material::{Material2D, Texture2D};
use scion::rendering::bidimensional::transform::{Position2D, Transform2D};

use scion::rendering::bidimensional::components::camera::Camera2D;
use scion::rendering::bidimensional::components::square::Square;
use scion::inputs::Inputs;

fn square() -> Square {
    Square::new(
        Position2D { x: 0., y: 0. },
        192.,
        Some([
            Position2D { x: 0., y: 0. },
            Position2D { x: 0., y: 1. },
            Position2D { x: 1., y: 1. },
            Position2D { x: 1., y: 0. },
        ]),
    )
}

#[system]
fn inputs(
    #[resource] inputs: &Inputs
) {
    if inputs.mouse().click_event() {
        log::info!("Clicked on {:?}, {:?}", inputs.mouse().x(), inputs.mouse().y());
    }
}
#[derive(Default)]
struct Layer;

impl SimpleGameLayer for Layer {
    fn on_start(&mut self, world: &mut World, resource: &mut Resources) {
        let square = (
            square(),
            Material2D::Texture(Texture2D::from_png(Path::new("Yo"))),
            Transform2D::new(Position2D { x: 192.0, y: 192. }, 1., 0.),
        );
        world.push(square);
        resource.insert(Camera2D::new(768., 768., 10.));
    }
}

fn main() {
    Scion::app()
        .with_system(inputs_system())
        .with_game_layer(GameLayer::weak::<Layer>())
        .run();
}
