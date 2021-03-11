use crate::renderer::{ScionRenderer, Renderable2D};
use miniquad::Context;
use legion::{Resources, World, Entity, IntoQuery};
use crate::renderer::bidimensional::triangle::Triangle;


pub struct Scion2D;

impl ScionRenderer for Scion2D {
    fn draw(&mut self, context: &mut Context, world: &mut World, _resource: &mut Resources) {
        let mut query_triangles = <(Entity, &Triangle)>::query();
        query_triangles.for_each(world,|_triangle|{
           Triangle::render(context)
        });
    }
}