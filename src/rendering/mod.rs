pub mod bidimensional;
pub mod color;
pub mod renderer_state;
use legion::{Resources, World};

use crate::rendering::bidimensional::scion2d::Scion2D;

use wgpu::{CommandEncoder, Device, Queue, SwapChainDescriptor, SwapChainTexture};

/// Trait to implement in order to create a renderer to use in a `Scion` application
pub trait ScionRenderer {
    fn update(
        &mut self,
        world: &mut World,
        resource: &mut Resources,
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        queue: &mut Queue,
    );
    fn render(
        &mut self,
        world: &mut World,
        resources: &mut Resources,
        frame: &SwapChainTexture,
        encoder: &mut CommandEncoder,
    );
}

/// Type of renderer to use to render the game.
pub enum RendererType {
    Scion2D,
    Custom(Box<dyn ScionRenderer>),
}

impl Default for RendererType {
    fn default() -> Self {
        RendererType::Scion2D
    }
}

impl RendererType {
    pub(crate) fn into_boxed_renderer(self) -> Box<dyn ScionRenderer> {
        match self {
            RendererType::Scion2D => Box::new(Scion2D::default()),
            RendererType::Custom(boxed) => boxed,
        }
    }
}
