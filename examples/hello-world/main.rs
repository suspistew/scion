use scion::application::Scion;
use scion::legion::{system, Resources, World};
use scion::utils::time::Time;
use scion::game_layer::{SimpleGameLayer, GameLayer};

use scion::renderer::bidimensional::triangle::Triangle;
use scion::renderer::bidimensional::material::{Material2D};
use scion::renderer::color::Color;
use scion::renderer::bidimensional::transform::{Transform2D, Position2D};



#[system(for_each)]
fn color(#[state] timer: &mut f32, #[resource] time: &Time, material: &mut Material2D, transform: &mut Transform2D) {
    *timer += time.delta_duration().as_secs_f32();
    if *timer > 0.01 {
        *timer = 0.;
        match material {
            Material2D::Color(color) => {
                let new_red = if color.red() < 255 { color.red() + 1 } else { 0 };
                color.replace(Color::new_rgb(new_red, color.green(), color.blue()));
            }
            _ => {}
        }
    }
    transform.append_angle(0.1);
}

#[derive(Default)]
struct Layer;

impl SimpleGameLayer for Layer {
    fn on_start(&mut self, world: &mut World, _resource: &mut Resources) {
        let triangle1 =
            (Triangle::default(),
             Material2D::Color(Color::new(0, 47, 110, 1.0)),
             Transform2D::new(Position2D { x: 0.0, y: 0.0 }, 0.5, 0.)
            );
        world.extend(vec![triangle1]);
    }
}

fn main() {
    Scion::app()
        .with_system(color_system(0.))
        .with_game_layer(GameLayer::weak::<Layer>())
        .run();
}