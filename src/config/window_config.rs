use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) title: String,
    pub(crate) fullscreen: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Default window".to_string(),
            fullscreen: false,
        }
    }
}

pub struct WindowConfigBuilder {
    config: WindowConfig
}

impl WindowConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Default::default()
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.config.title = title;
        self
    }

    pub fn with_width(mut self, width: i32) -> Self {
        self.config.width = width;
        self
    }

    pub fn with_height(mut self, height: i32) -> Self {
        self.config.height = height;
        self
    }

    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.config.fullscreen = fullscreen;
        self
    }

    pub fn get(self) -> WindowConfig {
        self.config
    }
}