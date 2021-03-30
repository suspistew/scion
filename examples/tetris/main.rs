use scion::{
    core::{
        components::{
            maths::{camera::Camera2D, transform::Transform2D},
            ui::ui_image::UiImage,
        },
        game_layer::{GameLayer, SimpleGameLayer},
        resources::time::{TimerType, Timers},
    },
    legion::{system, Resources, World},
    utils::file::app_base_path,
    Scion,
};
use scion::core::components::ui::font::Font;
use scion::core::components::ui::ui_text::UiText;


#[derive(Default)]
struct LayerA;

impl SimpleGameLayer for LayerA {
    fn on_start(&mut self, world: &mut World, resource: &mut Resources) {
        let path = app_base_path()
            .expect("")
            .join("assets")
            .join("tetris")
            .join("ui.png")
            .to_str()
            .expect("")
            .to_string();
        let mut t = Transform2D::default();
        t.set_layer(0);
        let image = UiImage::new(544., 704., path);

        world.push((image, t));
        resource.insert(Camera2D::new(544., 704., 10.));

        // First we add an UiText to the world
        let font = Font::Bitmap {
            texture_path: app_base_path()
                .expect("")
                .join("assets")
                .join("tetris")
                .join("font.png").to_str().expect("").to_string(),
            chars: "0123456789ACEOPRSULI".to_string(),
            texture_columns: 20.,
            texture_lines: 1.,
            width: 21.,
            height: 27.,
        };

        let txt = UiText::new("009287".to_string(), font);
        let mut transform = Transform2D::default();
        transform.append_translation(382., 250.);
        transform.set_layer(2);

        world.push((txt, transform));
    }
}

fn main() {
    Scion::app()
        .with_game_layer(GameLayer::weak::<LayerA>())
        .run();
}
