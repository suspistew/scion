//! Everything that is relative to rendering to the window (Like renderable components, camera, transforms..)
use legion::{Resources, World};
use wgpu::{CommandEncoder, Device, Queue, SwapChainDescriptor, SwapChainTexture};

use crate::{config::scion_config::ScionConfig, rendering::bidimensional::scion2d::Scion2D};

pub mod bidimensional;
pub(crate) mod renderer_state;
pub(crate) mod shaders;

/// Trait to implement in order to create a renderer to use in a `Scion` application
pub trait ScionRenderer {
    /// Will be called first, before render, each time the window request redraw.
    fn update(
        &mut self,
        world: &mut World,
        resources: &mut Resources,
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        queue: &mut Queue,
    );

    /// Will be called after render, each time the window request redraw.
    fn render(
        &mut self,
        world: &mut World,
        config: &ScionConfig,
        frame: &SwapChainTexture,
        encoder: &mut CommandEncoder,
    );
}

/// Type of renderer to use to render the game.
pub enum RendererType {
    /// Internal 2D Renderer. Will render everything that is in [`bidimensional`]
    Scion2D,
    /// Provide your own renderer
    Custom(Box<dyn ScionRenderer>),
}

impl Default for RendererType {
    fn default() -> Self { RendererType::Scion2D }
}

impl RendererType {
    pub(crate) fn into_boxed_renderer(self) -> Box<dyn ScionRenderer> {
        match self {
            RendererType::Scion2D => Box::new(Scion2D::default()),
            RendererType::Custom(boxed) => boxed,
        }
    }
}
