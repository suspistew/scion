use std::fmt::{Display, Formatter};

use rand::{thread_rng, Rng};
use scion::{
    core::{
        components::{
            material::Material,
            maths::{
                collider::{Collider, ColliderMask, ColliderType},
                transform::Transform,
            },
            tiles::{sprite::Sprite, tileset::Tileset},
        },
        game_layer::SimpleGameLayer,
        legion_ext::{ScionResourcesExtension, ScionWorldExtension},
        resources::{
            asset_manager::AssetRef,
            sound::{Sound, SoundLoadingType},
        },
    },
    legion::{Resources, World},
};
use winit::window::CursorIcon;

use crate::utils::{ball_animations, ball_asset, ball_bounce_effect};

#[derive(Debug)]
pub enum BallDirection {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug)]
pub enum LineDirection {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CursorState {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
    ROW,
    COLUMN,
}

impl Display for BallDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

#[derive(Default)]
pub struct Ball {
    pub direction: Option<BallDirection>,
}

pub struct JezzBallAssets {
    ball_asset: AssetRef<Material>,
}

#[derive(Default)]
pub struct JezzBallLayer;

impl SimpleGameLayer for JezzBallLayer {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        world.add_default_camera();

        let ball_asset = resources.assets().register_tileset(Tileset::new(ball_asset(), 4, 4, 28));
        let assets = JezzBallAssets { ball_asset };

        // Creating the level

        init_balls(world, &assets);
        add_border(world);

        let _r = resources.audio().register_sound(
            "BOUNCE",
            Sound::new(ball_bounce_effect(), SoundLoadingType::AlwaysInMemory),
        );

        // default mouse cursor
        resources.window().set_cursor(CursorIcon::NResize);

        resources.insert(assets);
    }
}

pub fn init_balls(world: &mut World, assets: &JezzBallAssets) {
    let x = thread_rng().gen_range(10..58) as f32;
    let y = thread_rng().gen_range(5..33) as f32;

    world.push((
        Transform::from_xyz(10. + x * 16., 10. + y * 16., 15),
        Sprite::new(0),
        assets.ball_asset.clone(),
        Ball::default(),
        Collider::new(
            ColliderMask::Bullet,
            vec![
                ColliderMask::Custom("BORDER_TOP".to_string()),
                ColliderMask::Custom("BORDER_CUSTOM_HORIZONTAL".to_string()),
                ColliderMask::Custom("BORDER_CUSTOM_VERTICAL".to_string()),
                ColliderMask::Custom("BORDER_BOTTOM".to_string()),
                ColliderMask::Custom("BORDER_LEFT".to_string()),
                ColliderMask::Custom("BORDER_RIGHT".to_string()),
            ],
            ColliderType::Square(28),
        ),
        ball_animations(),
    ));
}

fn add_border(world: &mut World) {
    world.push((
        Transform::from_xy(2.0, 0.),
        Collider::new(
            ColliderMask::Custom("BORDER_TOP".to_string()),
            vec![ColliderMask::None],
            ColliderType::Rectangle(1104, 10),
        ),
    ));

    world.push((
        Transform::from_xy(2.0, 619.),
        Collider::new(
            ColliderMask::Custom("BORDER_BOTTOM".to_string()),
            vec![ColliderMask::None],
            ColliderType::Rectangle(1104, 10),
        ),
    ));

    world.push((
        Transform::from_xy(1.0, 10.),
        Collider::new(
            ColliderMask::Custom("BORDER_LEFT".to_string()),
            vec![ColliderMask::None],
            ColliderType::Rectangle(10, 609),
        ),
    ));

    world.push((
        Transform::from_xy(1098., 10.),
        Collider::new(
            ColliderMask::Custom("BORDER_RIGHT".to_string()),
            vec![ColliderMask::None],
            ColliderType::Rectangle(10, 609),
        ),
    ));
}
