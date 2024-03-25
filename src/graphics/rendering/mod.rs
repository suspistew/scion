//! Everything that is relative to graphics to the window (Like renderable components, camera, transforms..)
use std::ops::Range;
use hecs::Entity;

use scion2d::Scion2D;
use wgpu::{BufferUsages, CommandEncoder, Device, Queue, SurfaceConfiguration, TextureView, util::BufferInitDescriptor};

use crate::core::world::GameData;
use crate::core::components::material::{Material, Texture, TextureArray};
use crate::core::components::color::Color;
use crate::core::components::maths::Pivot;
use crate::graphics::rendering::gl_representations::GlUniform;

use crate::utils::maths::Vector;

pub(crate) mod gl_representations;
pub(crate) mod renderer_state;
pub(crate) mod scion2d;
pub(crate) mod shaders;
mod rendering_texture_management;
pub(crate) mod scion2d_renderer;
pub(crate) mod rendering_thread;

/// Trait to implement in order to create a renderer to use in a `Scion` application
pub trait ScionRenderer {
    fn start(&mut self, device: &Device, surface_config: &SurfaceConfiguration);

    /// Will be called first, before render, each time the window request redraw.
    fn update(
        &mut self,
        data: &mut GameData,
        device: &Device,
        surface_config: &SurfaceConfiguration,
        queue: &mut Queue,
    );

    /// Will be called after render, each time the window request redraw.
    fn render(
        &mut self,
        data: &mut GameData,
        default_background: &Option<Color>,
        texture_view: TextureView,
        encoder: &mut CommandEncoder,
    );
}

/// Type of renderer to use to render the game.
#[derive(Default)]
pub enum RendererType {
    /// Internal 2D Renderer. Will render everything that is in [`bidimensional`]
    #[default]
    Scion2D,
    /// Provide your own renderer
    Custom(Box<dyn ScionRenderer + Send>),
}


impl RendererType {
    pub(crate) fn into_boxed_renderer(self) -> Box<dyn ScionRenderer + Send> {
        match self {
            RendererType::Scion2D => Box::<Scion2D>::default(),
            RendererType::Custom(boxed) => boxed,
        }
    }
}

pub(crate) trait Renderable2D {
    fn vertex_buffer_descriptor(&mut self, material: Option<&Material>) -> BufferInitDescriptor;
    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor;
    fn range(&self) -> Range<u32>;
    fn topology() -> wgpu::PrimitiveTopology;
    fn dirty(&self) -> bool;
    fn set_dirty(&mut self, is_dirty: bool);
    fn get_pivot_offset(&self, _material: Option<&Material>) -> Vector { Vector::default() }
    fn get_pivot(&self) -> Pivot { Pivot::TopLeft }
}

pub(crate) trait RenderableUi: Renderable2D {}


#[derive(Debug)]
pub enum RenderingUpdate {
    DiffuseBindGroup {
        path: String,
        data: DiffuseBindGroupUpdate,
    },
    TransformUniform {
        entity: Entity,
        uniform: GlUniform,
    },
    VertexBuffer{
        label: String,
        contents: Vec<u8>,
        usage: BufferUsages

    },
    IndexBuffer{
        label: String,
        contents: Vec<u8>,
        usage: BufferUsages

    },
    TilemapBuffer,
    UiComponentBuffer,
}

#[derive(Debug)]
pub enum DiffuseBindGroupUpdate {
    ColorBindGroup(Texture),
    TextureBindGroup(Texture),
    TilesetBindGroup(TextureArray),
}

#[derive(Debug)]
pub struct RenderingInfos {
    pub(crate) layer: usize,
    pub(crate) range: Range<u32>,
    pub(crate) entity: Entity,
    pub(crate) texture_path: Option<String>,
    pub(crate) type_name: String,
}
