use std::path::Path;

use scion::application::Scion;
use scion::game_layer::{GameLayer, SimpleGameLayer};
use scion::legion::{Resources, system, World};
use scion::rendering::bidimensional::components::triangle::Triangle;
use scion::rendering::bidimensional::material::{Material2D, Texture2D};
use scion::rendering::bidimensional::transform::{Position2D, Transform2D};
use scion::rendering::color::Color;
use scion::utils::time::Time;

fn triangle() -> Triangle {
    Triangle::new(
        [
            Position2D { x: -0.5, y: -0.5 },
            Position2D { x: 0.5, y: -0.5 },
            Position2D { x: 0., y: 0.5 },
        ],
        Some([
            Position2D { x: 0., y: 1. },
            Position2D { x: 1., y: 1. },
            Position2D { x: 0.5, y: 0. }
        ]),
    )
}

#[system(for_each)]
fn color(
    #[state] timer: &mut f32,
    #[resource] time: &Time,
    material: &mut Material2D,
    transform: &mut Transform2D,
) {
    *timer += time.delta_duration().as_secs_f32();
    if *timer > 0.01 {
        *timer = 0.;
        match material {
            Material2D::Color(color) => {
                let new_red = if color.red() < 255 {
                    color.red() + 1
                } else {
                    0
                };
                color.replace(Color::new_rgb(new_red, color.green(), color.blue()));
            }
            _ => {}
        }
        transform.append_angle(0.1);
    }
}

#[derive(Default)]
struct Layer;

impl SimpleGameLayer for Layer {
    fn on_start(&mut self, world: &mut World, _resource: &mut Resources) {
        let triangle1 = (
            triangle(),
            Material2D::Texture(Texture2D::from_png(Path::new("Yo"))),
            Transform2D::new(Position2D { x: -1.0, y: 0.0 }, 0.5, 0.),
        );
        let triangle2 = (
            triangle(),
            Material2D::Texture(Texture2D::from_png(Path::new("Yo"))),
            Transform2D::new(Position2D { x: 1.0, y: 0.0 }, 0.5, 0.),
        );
        world.extend(vec![triangle1, triangle2]);
    }
}

fn main() {
    Scion::app()
        .with_system(color_system(0.))
        .with_game_layer(GameLayer::weak::<Layer>())
        .run();
}
