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

#[system]
fn test(#[resource] timers: &mut Timers) {
    if !timers.exists("test") {
        timers.add_timer("test", TimerType::Manual, 5.);
    }

    let test = timers.get_timer("test").unwrap();
    log::info!(
        "test elapsed {:?}, ended {:?}",
        test.elapsed(),
        test.ended()
    );
}

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

        let path = app_base_path()
            .expect("")
            .join("assets")
            .join("taquin.png")
            .to_str()
            .expect("")
            .to_string();
        let mut t = Transform2D::default();
        t.set_layer(1);
        let image = UiImage::new(300., 300., path);

        world.push((image, t));
    }
}

fn main() {
    Scion::app()
        .with_game_layer(GameLayer::weak::<LayerA>())
        .with_system(test_system())
        .run();
}
