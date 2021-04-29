use scion::{
    config::{scion_config::ScionConfigBuilder, window_config::WindowConfigBuilder},
    core::{
        components::{
            material::Material,
            maths::{
                camera::Camera,
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
use scion::core::resources::asset_manager::AssetManager;
use scion::core::components::color::Color;

#[derive(Default)]
struct Layer;

impl SimpleGameLayer for Layer {
    fn on_start(&mut self, world: &mut World, resource: &mut Resources) {
        let asset_ref = {
            let mut asset_manager = resource.get_mut::<AssetManager>().expect("");
            asset_manager.register_material(Material::Color(Color::new(1, 255, 1, 0.5)))
        };
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
            asset_ref,
            Transform::new(Coordinates::new(200., 200.), 1., 0.),
        );
        world.push(square);
        resource.insert(Camera::new_with_position(
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
