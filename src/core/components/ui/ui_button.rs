use crate::core::components::color::Color;
use crate::core::components::maths::padding::Padding;
use crate::core::components::ui::Focusable;
use crate::core::components::ui::font::Font;
use crate::core::resources::asset_manager::AssetRef;

pub struct UiButton {
    text: String,
    width: usize,
    height: usize,
    background_color: Option<Color>,
    font_ref: AssetRef<Font>,
    /// font size when using a TrueType font
    font_size: usize,
    /// font color when using a TrueType font
    font_color: Option<Color>,
    tab_index: usize,
    padding: Padding,
    pub(crate) dirty: bool
}

impl UiButton {
    pub fn new(width: usize, height: usize, font_ref: AssetRef<Font>) -> Self {
        Self {
            text: "".to_string(),
            width,
            height,
            background_color: None,
            font_ref,
            font_size: 0,
            font_color: None,
            tab_index: 0,
            padding: Padding::default(),
            dirty: true
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
    pub fn font_color(&self) -> Option<Color> {
        self.font_color.clone()
    }
    pub fn background_color(&self) -> Option<Color> {
        self.background_color.clone()
    }
    pub fn font_size(&self) -> usize {
        self.font_size
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn padding(&self) -> Padding {
        self.padding.clone()
    }
    pub fn font_ref(&self) -> AssetRef<Font> {
        self.font_ref.clone()
    }


    pub fn with_font_size(mut self, font_size: usize) -> Self{
        self.font_size = font_size;
        self
    }

    pub fn with_font_color(mut self, color: Color) -> Self{
        self.font_color = Some(color);
        self
    }

    pub fn with_background_color(mut self, color: Color) -> Self{
        self.background_color = Some(color);
        self
    }

    pub fn with_tab_index(mut self, tab_index: usize) -> Self{
        self.tab_index = tab_index;
        self
    }

    pub fn with_padding(mut self, padding: Padding) -> Self{
        self.padding = padding;
        self
    }

    pub fn set_text(&mut self, text: String) {
        if text.ne(&self.text) {
            self.text = text;
            self.dirty = true;
        }
    }

}

impl Focusable for UiButton {
    fn tab_index(&self) -> usize {
        self.tab_index
    }

    fn set_tab_index(&mut self, tab_index: usize) {
        self.tab_index = tab_index;
    }
}