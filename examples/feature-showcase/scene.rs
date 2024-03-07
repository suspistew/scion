use std::time::Duration;

use hecs::Entity;

use scion::core::components::{Square, Triangle};
use scion::core::components::animations::{Animation, AnimationModifier, Animations};
use scion::core::components::color::Color;
use scion::core::components::material::Material;
use scion::core::components::maths::coordinates::Coordinates;
use scion::core::components::maths::Pivot;
use scion::core::components::maths::transform::TransformBuilder;
use scion::core::components::shapes::line::Line;
use scion::core::components::shapes::rectangle::Rectangle;
use scion::core::components::tiles::sprite::Sprite;
use scion::core::components::tiles::tileset::Tileset;
use scion::core::scene::Scene;
use scion::core::world::{GameData, World};
use scion::utils::file::app_base_path;

#[derive(Default)]
pub struct ShowCaseScene{
    pub square_entity: Option<Entity>,
    pub triangle_entity: Option<Entity>,
    pub sprite_entity: Option<Entity>,
    pub rect_entity: Option<Entity>,
    pub line_entity: Option<Entity>
}

impl Scene for ShowCaseScene{
    fn on_start(&mut self, data: &mut GameData) {
        data.add_default_camera();
        self.square_entity = Some(create_square(data));
        self.triangle_entity = Some(create_triangle(data));
        self.sprite_entity = Some(create_sprite(data));
        self.rect_entity = Some(create_rect(data));
        self.rect_entity = Some(create_line(data));
    }
}

fn create_square(data: &mut GameData) -> Entity {
    data.push((
        TransformBuilder::new().with_xy(50., 50.).with_scale(1.5).build(),
        Square::new(50., None).pivot(Pivot::Center),
        Material::Color(Color::new_rgb(200,0,0)),
        Animations::single("infinite-rotate",
                           Animation::looping(Duration::from_millis(2000),
                                              vec![AnimationModifier::transform(50, None, None, Some(3.))]))
    ))
}

fn create_triangle(data: &mut GameData) -> Entity {
    data.push((
        TransformBuilder::new().with_xy(150., 50.).with_scale(1.5).build(),
        Triangle::new([Coordinates::new(25., 0.), Coordinates::new(50., 50.), Coordinates::new(0., 50.)],
                      Some([Coordinates::new(0.5, 0.), Coordinates::new(1., 1.), Coordinates::new(0., 1.)])).pivot(Pivot::Center),
        Material::Color(Color::new_rgb(0,200,0)),
        Animations::single("infinite-rotate",
                           Animation::looping(Duration::from_millis(2000),
                                              vec![AnimationModifier::transform(50, None, None, Some(3.))]))
    ))
}

fn create_sprite(data: &mut GameData) -> Entity {
    let tileset_ref = data.assets_mut().register_tileset(
        Tileset::new(app_base_path().join("examples/taquin/assets/taquin.png").get(), 1,1,50)
    );

    data.push((
        TransformBuilder::new().with_xy(250., 50.).with_scale(1.5).build(),
        tileset_ref,
        Sprite::new(0).pivot(Pivot::Center),
        Animations::single("infinite-rotate",
                           Animation::looping(Duration::from_millis(2000),
                                              vec![AnimationModifier::transform(50, None, None, Some(3.))]))
    ))
}

fn create_rect(data: &mut GameData) -> Entity {
    data.push((
        TransformBuilder::new().with_xy(400., 50.).with_scale(1.5).build(),
        Rectangle::new(50.,100., None).pivot(Pivot::Center),
        Material::Color(Color::new_rgb(0,0,200)),
        Animations::single("infinite-rotate",
                           Animation::looping(Duration::from_millis(2000),
                                              vec![AnimationModifier::transform(50, None, None, Some(3.))]))
    ))
}

fn create_line(data: &mut GameData) -> Entity {
    data.push((
        TransformBuilder::new().with_xy(550., 50.).with_scale(1.5).build(),
        Line::new([Coordinates::new(0.,0.), Coordinates::new(100., 0.)]).pivot(Pivot::Center),
        Material::Color(Color::new_rgb(0,200,200)),
        Animations::single("infinite-rotate",
                           Animation::looping(Duration::from_millis(2000),
                                              vec![AnimationModifier::transform(50, None, None, Some(3.))]))
    ))
}