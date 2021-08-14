use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use winit::{
    dpi::Size,
    window::{WindowAttributes, WindowBuilder},
};

use crate::{config::scion_config::ScionConfig, core::components::color::Color};

/// Main configuration for the game window
/// Please use [`WindowConfigBuilder`] if you want to build if from code.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    /// Enables fullscreen mode
    pub(crate) fullscreen: bool,
    /// Default window width and height in pixels.
    pub(crate) dimensions: Option<(u32, u32)>,
    /// Minimum window width and height in pixels.
    pub(crate) min_dimensions: Option<(u32, u32)>,
    /// Maximum window width and height in pixels.
    pub(crate) max_dimensions: Option<(u32, u32)>,
    /// Whether to display the window, Use full for loading
    pub(crate) visibility: bool,
    /// The path relative to the game executable of the window icon.
    pub(crate) icon: Option<PathBuf>,
    /// Whether the window should always be on top of other windows.
    pub(crate) always_on_top: bool,
    /// Whether the window should have borders and bars.
    pub(crate) decorations: bool,
    /// Whether the window should be maximized upon creation.
    pub(crate) maximized: bool,
    /// If the user can resize the window
    pub(crate) resizable: bool,
    /// If the window should be able to be transparent.
    pub(crate) transparent: bool,
    /// Default background color of each frame in the window
    pub(crate) default_background_color: Option<Color>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            fullscreen: false,
            dimensions: Some((1024, 768)),
            min_dimensions: Some((500, 480)),
            max_dimensions: None,
            visibility: true,
            icon: None,
            always_on_top: false,
            decorations: true,
            maximized: false,
            resizable: true,
            transparent: false,
            default_background_color: None,
        }
    }
}

impl WindowConfig {
    pub(crate) fn into(self, scion_config: &ScionConfig) -> WindowBuilder {
        let mut builder = WindowBuilder::new();

        builder.window = WindowAttributes {
            title: scion_config.app_name.clone(),
            fullscreen: None,
            inner_size: self.dimensions.map(|d| d.into()).map(Size::Logical),
            min_inner_size: self.min_dimensions.map(|d| d.into()).map(Size::Logical),
            max_inner_size: self.max_dimensions.map(|d| d.into()).map(Size::Logical),
            visible: self.visibility,
            window_icon: None,
            always_on_top: self.always_on_top,
            decorations: self.decorations,
            maximized: self.maximized,
            resizable: self.resizable,
            transparent: self.transparent,
            position: None,
        };
        builder
    }
}

pub struct WindowConfigBuilder {
    config: WindowConfig,
}

impl WindowConfigBuilder {
    pub fn new() -> Self { Self { config: Default::default() } }

    pub fn with_dimensions(mut self, dimensions: (u32, u32)) -> Self {
        self.config.dimensions = Some(dimensions);
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.config.resizable = resizable;
        self
    }

    pub fn with_default_background_color(mut self, color: Option<Color>) -> Self {
        self.config.default_background_color = color;
        self
    }

    pub fn get(self) -> WindowConfig { self.config }
}
