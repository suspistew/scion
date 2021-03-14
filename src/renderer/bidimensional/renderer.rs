use crate::renderer::ScionRenderer;
use miniquad::Context;
use legion::{Resources, World, Entity, IntoQuery};
use crate::renderer::bidimensional::triangle::Triangle;
use crate::renderer::bidimensional::material::Material2D;
use crate::renderer::bidimensional::transform::Transform2D;
use log::info;

pub trait Renderable2D {
    fn render(&self,
              context: &mut Context,
              material: Option<&Material2D>,
              transform: &Transform2D);
}

pub struct Scion2D;

impl ScionRenderer for Scion2D {
    fn draw(&mut self, context: &mut Context, world: &mut World, _resource: &mut Resources) {
        context.begin_default_pass(Default::default());
        let mut query_triangles = <(Entity, &Triangle, &Material2D, &Transform2D)>::query();
        query_triangles.for_each(world, |(_e, triangle, material, transform)| {
            info!(
                "rendering triangle {:?}", transform
            );
            triangle.render(context, Some(material), transform)
        });
        context.end_render_pass();
        context.commit_frame();
    }
}