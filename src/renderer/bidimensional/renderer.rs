use crate::renderer::{ScionRenderer, Renderable2D};
use miniquad::Context;
use legion::{Resources, World, Entity, IntoQuery};
use crate::renderer::bidimensional::triangle::Triangle;
use crate::renderer::bidimensional::material::Material2D;


pub struct Scion2D;

impl ScionRenderer for Scion2D {
    fn draw(&mut self, context: &mut Context, world: &mut World, _resource: &mut Resources) {
        let mut query_triangles = <(Entity, &Triangle, &Material2D)>::query();
        query_triangles.for_each(world,|(_e, _t, m)|{
           Triangle::render(context, Some(m))
        });
    }
}