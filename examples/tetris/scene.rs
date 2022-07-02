use hecs::Entity;
use scion::{
    core::{
        components::{
            maths::transform::Transform,
            tiles::tileset::Tileset,
            ui::{font::Font, ui_image::UiImage, ui_text::UiText},
        },
        resources::time::TimerType,
        scene::Scene,
    },
};
use scion::core::world::World;

use crate::{asset_path, resources::TetrisResource};

#[derive(Default)]
pub struct MainScene {
    score: Option<Entity>,
}

impl Scene for MainScene {
    fn on_start(&mut self, world: &mut World) {
        add_main_ui_mask(world);
        add_ui_top_overflow(world);
        self.score = Some(add_score_ui(world));
        world.add_default_camera();
        let _r = world.timers().add_timer("piece", TimerType::Cyclic, 0.5);
        let _r = world.timers().add_timer("action_reset_timer", TimerType::Manual, 0.2);
        let mut tetris = TetrisResource::default();
        tetris.asset = Some(world.assets_mut().register_tileset(Tileset::new(
            asset_path().join("blocs.png").get(),
            8,
            1,
            32,
        )));
        world.insert_resource(tetris);
    }

    fn on_update(&mut self, world: &mut World) {
        let score = world.get_resource::<TetrisResource>().unwrap().score;
        world.entry_mut::<&mut UiText>(self.score.unwrap())
            .unwrap()
            .set_text(format!("{:05}", score))
    }
}

fn add_score_ui(world: &mut World) -> Entity {
    // First we add an UiText to the world
    let font = Font::Bitmap {
        texture_path: asset_path().join("font.png").get(),
        chars: "0123456789ACEOPRSULI".to_string(),
        texture_columns: 20.,
        texture_lines: 1.,
        width: 21.,
        height: 27.,
    };

    let txt = UiText::new("SCORE".to_string(), font.clone());
    let mut transform = Transform::default();
    transform.append_translation(394., 250.);
    transform.set_z(2);

    world.push((txt, transform));

    let txt = UiText::new("".to_string(), font);
    let mut transform = Transform::default();
    transform.append_translation(394., 290.);
    transform.set_z(2);
    world.push((txt, transform))
}

fn add_main_ui_mask(world: &mut World) {
    let path = asset_path().join("ui.png").get();
    let image = UiImage::new(544., 704., path);

    let mut t = Transform::default();
    t.set_z(0);
    world.push((image, t));
}

fn add_ui_top_overflow(world: &mut World) {
    let path = asset_path().join("ui_overflow_top.png").get();
    let image = UiImage::new(324., 32., path);

    let mut t = Transform::default();
    t.set_z(2);
    t.append_translation(32., 0.);
    world.push((image, t));
}
