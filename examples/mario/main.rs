mod character_control_system;
mod collisions_system;

use hecs::Entity;
use std::{path::Path, str::from_utf8};

use scion::core::world::{GameData, World};
use scion::{
    config::{scion_config::ScionConfigBuilder, window_config::WindowConfigBuilder},
    core::{
        components::{
            color::Color,
            material::Material,
            maths::{
                camera::Camera,
                collider::{Collider, ColliderMask, ColliderType},
                hierarchy::Parent,
                transform::Transform,
            },
            shapes::rectangle::Rectangle,
            Square,
        },
        scene::Scene,
    },
    utils::file::{app_base_path, read_file},
    Scion,
};

use crate::{character_control_system::move_char_system, collisions_system::collider_system};

pub const MAX_VELOCITY: i32 = 10;

#[derive(Default)]
pub struct Hero {
    pub velocity: i32,
    pub gravity: i32,
    pub landed: bool,
}

#[derive(Default)]
pub struct MainScene {
    hero: Option<Entity>,
    map: Vec<Vec<usize>>,
}

impl Scene for MainScene {
    fn on_start(&mut self, data: &mut GameData) {
        add_background(data);
        self.hero = Some(add_character(data));
        self.map = add_level_data(data);
        let mut camera_transform = Transform::from_xy(-202., -320.);
        camera_transform.set_global_translation_bounds(Some(0.), Some(2060.), Some(0.), Some(0.));
        data.push((
            Camera::new(500., 640.),
            camera_transform,
            Parent(self.hero.expect("Hero is mandatory")),
        ));
    }
    fn late_update(&mut self, data: &mut GameData) {
        let hero = data.entry_mut::<(&mut Hero, &mut Transform)>(self.hero.unwrap()).unwrap();
        if hero.0.velocity != 0 {
            hero.1.append_x(hero.0.velocity as f32 / MAX_VELOCITY as f32 * 2.5);
        }

        if hero.0.gravity != 0 {
            hero.1.append_y(hero.0.gravity as f32 / 3.);
            let mut line = (hero.1.global_translation().y() / 64.) as usize;
            if (hero.1.global_translation().y() % 64.) as usize > 0 {
                line += 1;
            }
            let column = (hero.1.global_translation().x() / 64.) as usize;
            let column2 = {
                let e = hero.1.global_translation().x() % 64.;
                if e > 0. {
                    column + 1
                } else {
                    column
                }
            };
            let v = self.map.get(line).unwrap().get(column).unwrap();
            if *v == 2 {
                hero.1.append_y(-1. * hero.1.global_translation().y() % 64.);
                hero.0.landed = true;
            } else {
                let v = self.map.get(line).unwrap().get(column2).unwrap();
                if *v == 2 {
                    hero.1.append_y(-1. * hero.1.global_translation().y() % 64.);
                    hero.0.landed = true;
                }
            }
        }
    }
}

fn add_level_data(data: &mut GameData) -> Vec<Vec<usize>> {
    let file = read_file(Path::new(&app_base_path().join("examples/mario/assets/level.csv").get()))
        .unwrap_or_default();
    let csv = from_utf8(file.as_slice()).expect("no");
    let level_data: Vec<Vec<usize>> = csv
        .split("\r\n")
        .map(|e| e.split(',').map(|f| f.parse::<usize>().unwrap()).collect())
        .collect();
    for (i, line) in level_data.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            let t = Transform::from_xy(j as f32 * 64., i as f32 * 64.);
            match *val {
                0 => {
                    data.push((
                        t,
                        Collider::new(
                            ColliderMask::Death,
                            vec![ColliderMask::None],
                            ColliderType::Square(64),
                        ),
                    ));
                }
                2 => {
                    data.push((
                        t,
                        Collider::new(
                            ColliderMask::Landscape,
                            vec![ColliderMask::None],
                            ColliderType::Square(64),
                        ),
                    ));
                }
                3 => {
                    data.push((
                        t,
                        Collider::new(
                            ColliderMask::Custom("Win".to_string()),
                            vec![ColliderMask::None],
                            ColliderType::Square(64),
                        ),
                    ));
                }
                _ => {}
            }
        }
    }
    level_data
}

fn add_background(data: &mut GameData) {
    let background = (
        Rectangle::new(2560., 640., None),
        Material::Texture(app_base_path().join("examples/mario/assets/level.png").get()),
        Transform::from_xyz(0., 0., 1),
    );
    data.push(background);
}

fn add_character(data: &mut GameData) -> Entity {
    data.push((
        Hero { velocity: 0, gravity: 1, landed: false },
        Collider::new(
            ColliderMask::Character,
            vec![ColliderMask::Landscape, ColliderMask::Death],
            ColliderType::Square(64),
        ),
        Square::new(64., None),
        Transform::from_xyz(256., 320., 2),
        Material::Color(Color::new_rgb(100, 120, 23)),
    ))
}

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(WindowConfigBuilder::new().with_dimensions((500, 640)).get())
            .get(),
    )
    .with_scene::<MainScene>()
    .with_system(move_char_system)
    .with_system(collider_system)
    .run();
}
