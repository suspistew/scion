use crate::core::components::ui::font::Font;
use crate::core::components::ui::ui_image::UiImage;
use crate::rendering::bidimensional::scion2d::{Renderable2D, RenderableUi};
use wgpu::{Device, BindGroupLayout, SwapChainDescriptor, RenderPipeline};
use wgpu::util::BufferInitDescriptor;
use std::ops::Range;

/// A component representing an Text in the UI.
pub struct UiText{
    text: String,
    font: Font,
    pub(crate) dirty: bool
}


impl UiText{
    pub fn new(text: String, font: Font) -> Self {
        Self {
            text,
            font,
            dirty: true
        }
    }

    pub fn text(&self)-> &String{
        &self.text
    }

    pub fn set_text(&mut self, text: String){
        self.text = text;
        self.dirty = true;
    }

    pub fn font(&self)-> &Font{
        &self.font
    }

}

/// `UiTextImage` is an internal component used to keep track of the character in case of a
/// bitmap font
#[derive(Debug)]
pub(crate) struct UiTextImage(pub(crate) UiImage);

impl Renderable2D for UiTextImage{
    fn vertex_buffer_descriptor(&self) -> BufferInitDescriptor {
        self.0.vertex_buffer_descriptor()
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        self.0.indexes_buffer_descriptor()
    }

    fn pipeline(&self, device: &Device, sc_desc: &SwapChainDescriptor, texture_bind_group_layout: &BindGroupLayout, transform_bind_group_layout: &BindGroupLayout) -> RenderPipeline {
        self.0.pipeline(device, sc_desc, texture_bind_group_layout, transform_bind_group_layout)
    }

    fn range(&self) -> Range<u32> {
        self.0.range()
    }
}

impl RenderableUi for UiTextImage{
    fn get_texture_path(&self) -> Option<String> {
        self.0.get_texture_path()
    }
}