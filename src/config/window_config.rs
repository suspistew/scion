use std::path::PathBuf;
use winit::window::{WindowBuilder, WindowAttributes};
use winit::dpi::Size;
use serde::{Deserialize, Serialize};

/// Main configuration for the game window
#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct WindowConfig {
    /// Window title
    pub title: String,
    /// Enables fullscreen mode
    #[serde(default)]
    pub fullscreen: bool,
    /// Default window width and height in pixels.
    #[serde(default)]
    pub dimensions: Option<(u32, u32)>,
    /// Minimum window width and height in pixels.
    #[serde(default)]
    pub min_dimensions: Option<(u32, u32)>,
    /// Maximum window width and height in pixels.
    #[serde(default)]
    pub max_dimensions: Option<(u32, u32)>,
    /// Whether to display the window, Use full for loading
    pub visibility: bool,
    /// The path relative to the game executable of the window icon.
    #[serde(default)]
    pub icon: Option<PathBuf>,
    /// Whether the window should always be on top of other windows.
    #[serde(default)]
    pub always_on_top: bool,
    /// Whether the window should have borders and bars.
    pub decorations: bool,
    /// Whether the window should be maximized upon creation.
    #[serde(default)]
    pub maximized: bool,
    /// If the user can resize the window
    pub resizable: bool,
    /// If the window should be able to be transparent.
    #[serde(default)]
    pub transparent: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Default Scion game".to_string(),
            fullscreen: false,
            dimensions: Some((1024, 768)),
            min_dimensions: Some((640, 480)),
            max_dimensions: None,
            visibility: true,
            icon: None,
            always_on_top: false,
            decorations: true,
            maximized: false,
            resizable: true,
            transparent: false
        }
    }
}

impl Into<WindowBuilder> for WindowConfig {
    fn into(self) -> WindowBuilder {
        let mut builder = WindowBuilder::new();

        builder.window = WindowAttributes {
            title: self.title.clone(),
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
        };
        builder
    }
}