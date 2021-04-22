use scion::{
    config::{scion_config::ScionConfigBuilder, window_config::WindowConfigBuilder},
    core::{
        components::{
            material::Material2D,
            maths::{
                camera::Camera2D,
                transform::{Coordinates, Transform},
            },
            Square,
        },
        game_layer::{GameLayer, SimpleGameLayer},
    },
    legion::{Resources, World},
    utils::file::app_base_path,
    Scion,
};

#[derive(Default)]
struct Layer;

impl SimpleGameLayer for Layer {
    fn on_start(&mut self, world: &mut World, resource: &mut Resources) {
        let p = app_base_path().expect("A base path is mandatory");
        let p = p.join("assets/taquin.png");
        let square = (
            Square::new(
                192.,
                Some([
                    Coordinates::new(0., 0.),
                    Coordinates::new(0., 1.),
                    Coordinates::new(1., 1.),
                    Coordinates::new(1., 0.),
                ]),
            ),
            Material2D::Texture(p.as_path().to_str().unwrap().to_string()),
            Transform::new(Coordinates::new(200., 200.), 1., 0.),
        );
        world.push(square);
        resource.insert(Camera2D::new_with_position(
            768.,
            768.,
            10.,
            Coordinates::new(0., 0.),
        ));
    }
}

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(WindowConfigBuilder::new().with_dimensions((768, 768)).get())
            .get(),
    )
    .with_game_layer(GameLayer::weak::<Layer>())
    .run();
}
