use scion::{
    core::{
        components::{
            maths::{camera::Camera2D, transform::Transform2D},
            ui::{font::Font, ui_image::UiImage, ui_text::UiText},
        },
        game_layer::SimpleGameLayer,
        resources::time::{TimerType, Timers},
    },
    legion::{Entity, Resources, World},
    utils::file::app_base_path,
};

use crate::{asset_path, resources::TetrisResource};

#[derive(Default)]
pub struct TetrisLayer {
    score: Option<Entity>,
}

impl SimpleGameLayer for TetrisLayer {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        add_main_ui_mask(world);
        add_ui_top_overflow(world);
        self.score = Some(add_score_ui(world));
        resources.insert(Camera2D::new(544., 704., 10.));
        let _r = resources
            .get_mut::<Timers>()
            .unwrap()
            .add_timer("piece", TimerType::Cyclic, 0.5);
        let _r = resources.get_mut::<Timers>().unwrap().add_timer(
            "action_reset_timer",
            TimerType::Manual,
            0.2,
        );
        resources.insert(TetrisResource::default());
    }

    fn update(&mut self, world: &mut World, resources: &mut Resources) {
        let tetris = resources.get::<TetrisResource>().unwrap();
        world
            .entry(self.score.unwrap())
            .unwrap()
            .get_component_mut::<UiText>()
            .unwrap()
            .set_text(format!("{:05}", tetris.score))
    }
}

fn add_score_ui(world: &mut World) -> Entity {
    // First we add an UiText to the world
    let font = Font::Bitmap {
        texture_path: app_base_path()
            .expect("")
            .join("assets")
            .join("tetris")
            .join("font.png")
            .to_str()
            .expect("")
            .to_string(),
        chars: "0123456789ACEOPRSULI".to_string(),
        texture_columns: 20.,
        texture_lines: 1.,
        width: 21.,
        height: 27.,
    };

    let txt = UiText::new("SCORE".to_string(), font.clone());
    let mut transform = Transform2D::default();
    transform.append_translation(394., 250.);
    transform.set_layer(2);

    world.push((txt, transform));

    let txt = UiText::new("".to_string(), font.clone());
    let mut transform = Transform2D::default();
    transform.append_translation(394., 290.);
    transform.set_layer(2);
    world.push((txt, transform))
}

fn add_main_ui_mask(world: &mut World) {
    let path = asset_path().join("ui.png").to_str().expect("").to_string();
    let image = UiImage::new(544., 704., path);

    let mut t = Transform2D::default();
    t.set_layer(0);
    world.push((image, t));
}

fn add_ui_top_overflow(world: &mut World) {
    let path = asset_path()
        .join("ui_overflow_top.png")
        .to_str()
        .expect("")
        .to_string();
    let image = UiImage::new(324., 32., path);

    let mut t = Transform2D::default();
    t.set_layer(2);
    t.append_translation(32., 0.);
    world.push((image, t));
}
