use std::fmt::{Display, Formatter};
use legion::{Entity, EntityStore, IntoQuery};
use legion::world::SubWorld;

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
        scene::Scene,
        legion_ext::{ScionResourcesExtension, ScionWorldExtension},
        resources::{
            asset_manager::AssetRef,
            sound::{Sound, SoundLoadingType},
        },
    },
    legion::{Resources, World},
};
use winit::window::CursorIcon;
use scion::core::components::color::Color;
use scion::core::components::shapes::rectangle::Rectangle;
use scion::core::components::tiles::tilemap::{TileInfos, Tilemap, TilemapInfo};
use scion::core::scene::SceneController;
use scion::core::resources::events::{PollConfiguration, SubscriberId};
use scion::core::resources::events::topic::TopicConfiguration;

use crate::utils::{ball_animations, ball_asset, ball_bounce_effect, cases_asset};
use scion::utils::maths::{Dimensions, Position, Vector};

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

pub struct Line {
    pub direction: LineDirection,
}

pub struct MainScene{
    tilemap: Option<Entity>,
    subscriber_id: Option<SubscriberId>,
    mouse_state: CursorState,
    state_before_border: Option<CursorState>,
}

impl Default for MainScene {
    fn default() -> Self { Self { tilemap: None, subscriber_id: None, mouse_state: CursorState::ROW, state_before_border: None } }
}

impl Scene for MainScene {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        world.add_default_camera();

        let ball_asset = resources.assets().register_tileset(Tileset::new(ball_asset(), 4, 4, 28));
        let assets = JezzBallAssets { ball_asset };

        // Creating the level

        for i in 0..200 {
            init_balls(world, &assets);
        }
        add_border(world);

        let _r = resources.audio().register_sound(
            "BOUNCE",
            Sound::new(ball_bounce_effect(), SoundLoadingType::AlwaysInMemory),
        );

        // default mouse cursor
        resources.window().set_cursor(CursorIcon::NResize);

        resources.insert(assets);

        self.tilemap = Some(add_tilemap(world, resources));
        let _r = resources.events().create_topic("TILEMAP_UPDATE", TopicConfiguration::default());
        self.subscriber_id = Some(
            resources.events().subscribe("TILEMAP_UPDATE", PollConfiguration::default()).unwrap(),
        );
    }

    fn on_update(&mut self, world: &mut World, resources: &mut Resources) {
        let (mut world_t, mut world_s) = world.split::<&mut Tilemap>();
        let mut tilemap_entry = world_t.entry_mut(*self.tilemap.as_ref().unwrap()).unwrap();
        let tilemap = tilemap_entry.get_component_mut::<Tilemap>().unwrap();
        let mut tilemap_modified = false;
        resources
            .events()
            .poll::<(usize, usize, usize, usize)>(&self.subscriber_id.as_ref().unwrap())
            .unwrap()
            .into_iter()
            .for_each(|(x_min, x_max, y_min, y_max)| {
                for x in x_min..=x_max {
                    for y in y_min..=y_max {
                        tilemap_modified = true;
                        tilemap.modify_sprite_tile(Position::new(x, y, 0), 2, &mut world_s);
                    }
                }
            });

        if tilemap_modified {
            let (world_b, mut world_c) = world_s.split::<(&Ball, &Transform)>();

            let ball_pos: Vec<(usize, usize)> = <(&Ball, &Transform)>::query()
                .iter(&world_b)
                .map(|(_, t)| {
                    (
                        (t.translation().x() as usize - 10) / 16,
                        (t.translation().y() as usize - 10) / 16,
                    )
                })
                .collect();
            let mut pathfinded_cases = [[false; 38]; 68];
            ball_pos.iter().for_each(|(pos_x, pos_y)| {
                let mut visited = [[false; 38]; 68];
                pathfind_from((*pos_x, *pos_y), &world_c, tilemap, &mut visited).iter().for_each(
                    |e| {
                        pathfinded_cases[e.0][e.1] = true;
                    },
                );
            });

            let mut open = 0;
            for i in 0..68 {
                for j in 0..38 {
                    if !pathfinded_cases[i][j] {
                        let tmp_pos = Position::new(i, j, 0);
                        tilemap.modify_sprite_tile(tmp_pos, 2, &mut world_c);
                    } else {
                        open += 1;
                    }
                }
            }

            if open < 150 {
                let mut controller = resources.get_mut::<SceneController>().unwrap();
                controller.switch::<MainScene>();
            }
        }
        let mut change_mouse_state = false;
        let mouse_pos = resources.inputs().mouse_xy();
        if !self.compute_mouse_on_border(mouse_pos, resources) {
            resources.inputs().on_right_click_released(|_pos_x, _pos_y| change_mouse_state = true);
            if change_mouse_state {
                let new_icon = self.compute_new_icon(resources);
                resources.window().set_cursor(new_icon);
            } else {
                if self.mouse_state != CursorState::ROW && self.mouse_state != CursorState::COLUMN {
                    self.mouse_state = self.state_before_border.as_ref().unwrap().clone();
                    self.state_before_border = None;
                    self.change_cursor(resources);
                }
            }
        }
        resources.inputs().on_left_click_pressed(|pos_x, pos_y| {
            let len = <(&Rectangle,)>::query().iter(world).collect::<Vec<(&Rectangle,)>>().len();

            if len == 0 && pos_x > 10. && pos_y > 10. {
                let x = (pos_x as usize - 10) / 16;
                let y = (pos_y as usize - 10) / 16;
                if self.mouse_state == CursorState::ROW {
                    world.push((
                        Transform::from_xyz(x as f32 * 16. + 10., y as f32 * 16. + 10., 20),
                        Rectangle::new(16., 16., None),
                        Material::Color(Color::new(255, 0, 0, 0.3)),
                        Line { direction: LineDirection::TOP },
                        Collider::new(
                            ColliderMask::Landscape,
                            vec![
                                ColliderMask::Bullet,
                                ColliderMask::Custom("BORDER_CUSTOM_HORIZONTAL".to_string()),
                                ColliderMask::Custom("BORDER_CUSTOM_VERTICAL".to_string()),
                                ColliderMask::Custom("BORDER_TOP".to_string()),
                                ColliderMask::Custom("BORDER_BOTTOM".to_string()),
                                ColliderMask::Custom("BORDER_LEFT".to_string()),
                                ColliderMask::Custom("BORDER_RIGHT".to_string()),
                            ],
                            ColliderType::Rectangle(16, 16),
                        ),
                    ));
                    world.push((
                        Transform::from_xyz(x as f32 * 16. + 10., y as f32 * 16. + 26., 20),
                        Rectangle::new(16., 16., None),
                        Material::Color(Color::new(0, 0, 255, 0.3)),
                        Line { direction: LineDirection::BOTTOM },
                        Collider::new(
                            ColliderMask::Landscape,
                            vec![
                                ColliderMask::Bullet,
                                ColliderMask::Custom("BORDER_CUSTOM".to_string()),
                                ColliderMask::Custom("BORDER_TOP".to_string()),
                                ColliderMask::Custom("BORDER_BOTTOM".to_string()),
                                ColliderMask::Custom("BORDER_LEFT".to_string()),
                                ColliderMask::Custom("BORDER_RIGHT".to_string()),
                            ],
                            ColliderType::Rectangle(16, 16),
                        ),
                    ));
                } else if self.mouse_state == CursorState::COLUMN {
                    world.push((
                        Transform::from_xyz(x as f32 * 16. + 10., y as f32 * 16. + 10., 20),
                        Rectangle::new(16., 16., None),
                        Material::Color(Color::new(255, 0, 0, 0.3)),
                        Line { direction: LineDirection::LEFT },
                        Collider::new(
                            ColliderMask::Landscape,
                            vec![
                                ColliderMask::Bullet,
                                ColliderMask::Custom("BORDER_CUSTOM".to_string()),
                                ColliderMask::Custom("BORDER_TOP".to_string()),
                                ColliderMask::Custom("BORDER_BOTTOM".to_string()),
                                ColliderMask::Custom("BORDER_LEFT".to_string()),
                                ColliderMask::Custom("BORDER_RIGHT".to_string()),
                            ],
                            ColliderType::Rectangle(16, 16),
                        ),
                    ));
                    world.push((
                        Transform::from_xyz(x as f32 * 16. + 26., y as f32 * 16. + 10., 20),
                        Rectangle::new(16., 16., None),
                        Material::Color(Color::new(0, 0, 255, 0.3)),
                        Line { direction: LineDirection::RIGHT },
                        Collider::new(
                            ColliderMask::Landscape,
                            vec![
                                ColliderMask::Bullet,
                                ColliderMask::Custom("BORDER_CUSTOM".to_string()),
                                ColliderMask::Custom("BORDER_TOP".to_string()),
                                ColliderMask::Custom("BORDER_BOTTOM".to_string()),
                                ColliderMask::Custom("BORDER_LEFT".to_string()),
                                ColliderMask::Custom("BORDER_RIGHT".to_string()),
                            ],
                            ColliderType::Rectangle(16, 16),
                        ),
                    ));
                }
            }
        });
    }

    fn on_stop(&mut self, world: &mut World, resources: &mut Resources) {
        world.remove(self.tilemap.unwrap());
        let mut to_delete: Vec<Entity> =
            <(Entity, &Line)>::query().iter(world).map(|(e, _)| *e).collect();
        to_delete.drain(0..to_delete.len()).for_each(|e| {
            world.remove(e);
        });
    }
}

impl MainScene{
    fn compute_mouse_on_border(
        &mut self,
        mouse_pos: (f64, f64),
        resources: &mut Resources,
    ) -> bool {
        let (x, y) = mouse_pos;
        if x < 11. {
            if self.state_before_border.is_none() {
                self.state_before_border = Some(self.mouse_state.clone())
            };
            resources.window().set_cursor(CursorIcon::EResize);
            self.mouse_state = CursorState::LEFT;
            true
        } else if x > 1097. {
            if self.state_before_border.is_none() {
                self.state_before_border = Some(self.mouse_state.clone())
            };
            resources.window().set_cursor(CursorIcon::WResize);
            self.mouse_state = CursorState::RIGHT;
            true
        } else if y < 11. {
            if self.state_before_border.is_none() {
                self.state_before_border = Some(self.mouse_state.clone())
            };
            resources.window().set_cursor(CursorIcon::SResize);
            self.mouse_state = CursorState::TOP;
            true
        } else if y > 618. {
            if self.state_before_border.is_none() {
                self.state_before_border = Some(self.mouse_state.clone())
            };
            resources.window().set_cursor(CursorIcon::NResize);
            self.mouse_state = CursorState::BOTTOM;
            true
        } else {
            false
        }
    }

    fn compute_new_icon(&mut self, _resources: &mut Resources) -> CursorIcon {
        if self.mouse_state == CursorState::ROW {
            self.mouse_state = CursorState::COLUMN;
            return CursorIcon::ColResize;
        }
        self.mouse_state = CursorState::ROW;
        CursorIcon::RowResize
    }

    fn change_cursor(&self, resource: &mut Resources) {
        let mut window = resource.window();
        window.set_cursor(match self.mouse_state {
            CursorState::TOP => CursorIcon::NResize,
            CursorState::BOTTOM => CursorIcon::NResize,
            CursorState::LEFT => CursorIcon::WResize,
            CursorState::RIGHT => CursorIcon::WResize,
            CursorState::ROW => CursorIcon::RowResize,
            CursorState::COLUMN => CursorIcon::ColResize,
        })
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
            ColliderType::Square(38),
        ).with_offset(Vector::new(-5., -5.)),
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


fn pathfind_from(
    pos: (usize, usize),
    world: &SubWorld,
    tilemap: &Tilemap,
    visited: &mut [[bool; 38]; 68],
) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let sides = compute_sides(pos);
    sides.iter().for_each(|side_pos| {
        if !visited[side_pos.0][side_pos.1] {
            visited[side_pos.0][side_pos.1] = true;
            let sprite = tilemap
                .retrieve_sprite_tile(&Position::new(side_pos.0, side_pos.1, 0), &world)
                .unwrap_or(2);
            if sprite != 2 {
                res.push(side_pos.clone());
                let mut recursive_call_result = pathfind_from(*side_pos, world, tilemap, visited);
                res.append(&mut recursive_call_result);
            }
        }
    });
    res
}

fn compute_sides(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    if pos.0 > 0 {
        res.push((pos.0 - 1, pos.1));
    }

    if pos.0 < 67 {
        res.push((pos.0 + 1, pos.1));
    }

    if pos.1 > 0 {
        res.push((pos.0, pos.1 - 1));
    }

    if pos.1 < 37 {
        res.push((pos.0, pos.1 + 1));
    }
    res
}

fn add_tilemap(world: &mut World, resources: &mut Resources) -> Entity {
    let cases_asset = resources.assets().register_tileset(Tileset::new(cases_asset(), 3, 2, 16));
    let infos = TilemapInfo::new(
        Dimensions::new(68, 38, 1),
        Transform::from_xyz(10., 10., 10),
        cases_asset,
    );
    Tilemap::create(infos, world, |position| {
        let line = position.y() % 2;
        let column = position.x() % 2;
        TileInfos::new(Some(line * 3 + column), None)
    })
}
