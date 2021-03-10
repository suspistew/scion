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