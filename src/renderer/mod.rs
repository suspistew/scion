pub mod bidimensional;
pub mod color;
use miniquad::Context;
use legion::{World, Resources};



/// Trait to implement in order to create a renderer to use in a `Scion` application
pub trait ScionRenderer {
    /// The draw method is called each frame
    fn draw(&mut self, context: &mut Context, world: &mut World, resource: &mut Resources);
}

/// Type of renderer to use to render the game.
pub enum RendererType {
    Scion2D,
    Custom(Box<dyn ScionRenderer>)
}

impl Default for RendererType{
    fn default() -> Self {
        RendererType::Scion2D
    }
}

impl RendererType{
    pub(crate) fn into_boxed_renderer(self) -> Box<dyn ScionRenderer>{
        match self{
            RendererType::Scion2D => { Box::new(bidimensional::renderer::Scion2D) }
            RendererType::Custom(boxed) => { boxed }
        }
    }
}