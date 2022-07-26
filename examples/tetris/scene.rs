use hecs::Entity;
use scion::core::world::{GameData, World};
use scion::core::{
    components::{
        maths::transform::Transform,
        tiles::tileset::Tileset,
        ui::{font::Font, ui_image::UiImage, ui_text::UiText},
    },
    resources::time::TimerType,
    scene::Scene,
};

use crate::{asset_path, resources::TetrisResource};

#[derive(Default)]
pub struct MainScene {
    score: Option<Entity>,
}

impl Scene for MainScene {
    fn on_start(&mut self, data: &mut GameData) {
        add_main_ui_mask(data);
        add_ui_top_overflow(data);
        self.score = Some(add_score_ui(data));
        data.add_default_camera();
        let _r = data.timers().add_timer("piece", TimerType::Cyclic, 0.5);
        let _r = data.timers().add_timer("action_reset_timer", TimerType::Manual, 0.2);
        let mut tetris = TetrisResource::default();
        tetris.asset = Some(data.assets_mut().register_tileset(Tileset::new(
            asset_path().join("blocs.png").get(),
            8,
            1,
            32,
        )));
        data.insert_resource(tetris);
    }

    fn on_update(&mut self, data: &mut GameData) {
        let score = data.get_resource::<TetrisResource>().unwrap().score;
        data.entry_mut::<&mut UiText>(self.score.unwrap())
            .unwrap()
            .set_text(format!("{:05}", score))
    }
}

fn add_score_ui(data: &mut GameData) -> Entity {
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

    data.push((txt, transform));

    let txt = UiText::new("".to_string(), font);
    let mut transform = Transform::default();
    transform.append_translation(394., 290.);
    transform.set_z(2);
    data.push((txt, transform))
}

fn add_main_ui_mask(data: &mut GameData) {
    let path = asset_path().join("ui.png").get();
    let image = UiImage::new(544., 704., path);

    let mut t = Transform::default();
    t.set_z(0);
    data.push((image, t));
}

fn add_ui_top_overflow(data: &mut GameData) {
    let path = asset_path().join("ui_overflow_top.png").get();
    let image = UiImage::new(324., 32., path);

    let mut t = Transform::default();
    t.set_z(2);
    t.append_translation(32., 0.);
    data.push((image, t));
}
