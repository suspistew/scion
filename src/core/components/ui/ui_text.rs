use std::ops::Range;

use wgpu::{util::BufferInitDescriptor, PrimitiveTopology};

use crate::{
    core::components::{
        material::Material,
        ui::{font::Font, ui_image::UiImage},
    },
    rendering::{Renderable2D, RenderableUi},
};
use crate::core::resources::asset_manager::AssetRef;

/// A component representing an Text in the UI.
pub struct UiText {
    text: String,
    font_ref: AssetRef<Font>,
    pub(crate) dirty: bool,
}

impl UiText {
    /// Creates a new `UiText` with `text` as default content and `font`
    pub fn new(text: String, font_ref: AssetRef<Font>) -> Self {
        Self { text, font_ref, dirty: true }
    }

    /// retrieves the content of this `UiText`
    pub fn text(&self) -> &String {
        &self.text
    }

    /// sets the content of this `UiText`
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.dirty = true;
    }

    /// retrieve the font of this `UiText`
    pub fn font_ref(&self) -> &AssetRef<Font> {
        &self.font_ref
    }
}

/// `UiTextImage` is an internal component used to keep track of the character in case of a
/// bitmap font
#[derive(Debug)]
pub(crate) struct UiTextImage(pub(crate) UiImage);

impl Renderable2D for UiTextImage {
    fn vertex_buffer_descriptor(&mut self, material: Option<&Material>) -> BufferInitDescriptor {
        self.0.vertex_buffer_descriptor(material)
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        self.0.indexes_buffer_descriptor()
    }

    fn range(&self) -> Range<u32> {
        self.0.range()
    }

    fn topology() -> PrimitiveTopology {
        wgpu::PrimitiveTopology::TriangleList
    }

    fn dirty(&self) -> bool {
        false
    }

    fn set_dirty(&mut self, _is_dirty: bool) {}
}

impl RenderableUi for UiTextImage {
    fn get_texture_path(&self) -> Option<String> {
        self.0.get_texture_path()
    }
}
