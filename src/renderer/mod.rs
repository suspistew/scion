pub mod bidimensional;
pub mod color;
pub mod renderer_state;
use legion::{World, Resources};
use winit::window::Window;
use winit::event::WindowEvent;
use crate::renderer::bidimensional::renderer::Scion2D;
use crate::renderer::bidimensional::triangle::triangle_pipeline;
use std::collections::HashMap;


/// Trait to implement in order to create a renderer to use in a `Scion` application
pub trait ScionRenderer {
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
        match self {
            RendererType::Scion2D => Box::new(Scion2D),
            RendererType::Custom(boxed) => { boxed }
        }
    }
}