use std::ops::Range;

use wgpu::util::BufferInitDescriptor;

use crate::{
    core::components::{
        material::Material,
        ui::{font::Font, ui_image::UiImage},
    },
    rendering::bidimensional::scion2d::{Renderable2D, RenderableUi},
};

/// A component representing an Text in the UI.
pub struct UiText {
    text: String,
    font: Font,
    pub(crate) dirty: bool,
}

impl UiText {
    pub fn new(text: String, font: Font) -> Self {
        Self {
            text,
            font,
            dirty: true,
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.dirty = true;
    }

    pub fn font(&self) -> &Font {
        &self.font
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
