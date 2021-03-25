use scion::game_layer::{GameLayer, GameLayerController, SimpleGameLayer};
use scion::legion::{Resources, system, World};
use scion::rendering::bidimensional::{Camera2D, Material2D, Coordinates, Transform2D};
use scion::rendering::bidimensional::components::Square;
use scion::Scion;
use scion::utils::file::app_base_path;
use scion::rendering::bidimensional::components::ui::ui_image::UiImage;

#[system]
fn test() {
    log::info!("Hello all");
}

#[derive(Default)]
struct LayerA;

impl SimpleGameLayer for LayerA {
    fn on_start(&mut self, world: &mut World, resource: &mut Resources) {
        let path = app_base_path().expect("").join("assets").join("tetris").join("ui.png").to_str().expect("").to_string();
        let mut t = Transform2D::default();
        t.set_layer(0);
        let image = UiImage::new(544., 704., path);

        world.push((image, t,));
        resource.insert(Camera2D::new(544., 704., 10.));

        let path = app_base_path().expect("").join("assets").join("taquin.png").to_str().expect("").to_string();
        let mut t = Transform2D::default();
        t.set_layer(1);
        let image = UiImage::new(300., 300., path);

        world.push((image, t,));

    }
}

fn main() {
    Scion::app()
        .with_game_layer(
            GameLayer::weak::<LayerA>()
        )
        .run();
}