use scion::Scion;
use scion::core::game_layer::{GameLayer, SimpleGameLayer};
use scion::legion::{World, Resources};
use scion::core::resources::asset_manager::AssetManager;
use scion::core::components::material::Material;
use scion::utils::file::app_base_path;
use scion::core::components::maths::camera::Camera;
use scion::core::components::maths::transform::{Transform, Coordinates};
use scion::core::components::tiles::sprite::Sprite;
use scion::core::components::Square;

#[derive(Default)]
pub struct WorldCup;

impl SimpleGameLayer for WorldCup{
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        let asset_ref = resources
            .get_mut::<AssetManager>()
            .expect("Asset Manager is mandatory")
            .register_material(Material::Texture(app_base_path()
                .join("examples/world-cup/assets/test.png")
                .get()));

        world.push((Square::new(500., None), Transform::new(Coordinates::new(100., 100.), 1.0, 0.), asset_ref));

        world.push((Camera::new(1024., 768., 10.), Transform::default()));

    }
}

fn main() {
    Scion::app()
        .with_game_layer(GameLayer::strong::<WorldCup>("main layer"))
        .run();
}