use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::config::logger_config::LoggerConfig;
use crate::config::window_config::WindowConfig;

/// Main configuration used by `crate::Scion` to configure the game.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScionConfig {
    /// Name of the application
    pub(crate) app_name: String,
    /// Logger configuration to use.
    pub(crate) logger_config: Option<LoggerConfig>,
    /// Window configuration to use.
    pub(crate) window_config: Option<WindowConfig>,
}

impl Default for ScionConfig {
    fn default() -> Self {
        Self {
            app_name: "Scion game".to_string(),
            logger_config: Some(Default::default()),
            window_config: Some(Default::default()),
        }
    }
}

pub struct ScionConfigBuilder{
    config: ScionConfig
}

impl ScionConfigBuilder{
    pub fn new() -> Self{
        Self {
            config:Default::default()
        }
    }

    pub fn with_app_name(mut self, app_name: String )-> Self{
        self.config.app_name = app_name;
        self
    }

    pub fn with_logger_config(mut self, logger_config: LoggerConfig)-> Self{
        self.config.logger_config = Some(logger_config);
        self
    }

    pub fn with_window_config(mut self, window_config: WindowConfig)-> Self{
        self.config.window_config = Some(window_config);
        self
    }

    pub fn get(self) -> ScionConfig {
        self.config
    }
}

pub(crate) struct ScionConfigReader;

impl ScionConfigReader {
    pub(crate) fn read_or_create_scion_toml() -> Result<ScionConfig, Error> {
        let path = Path::new("Scion.toml");
        let path_exists = path.exists();

        Ok(if !path_exists {
            println!("Couldn't find `Scion.toml` configuration file. Generating a new one");
            let config = ScionConfig::default();
            let mut file = File::create(path)?;
            file.write_all(toml::to_vec(&config).unwrap().as_slice())?;
            config
        } else {
            let mut scion_config = File::open(path)?;
            let mut bytes = Vec::new();
            scion_config.read_to_end(&mut bytes)?;
            let config = toml::from_slice(bytes.as_slice())?;
            config
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_scion_toml() {
        // Delete scion.toml before the test
        let path = Path::new("Scion.toml");
        let _r = std::fs::remove_file(path);

        let config = ScionConfigReader::read_or_create_scion_toml();
        assert!(config.is_ok());
    }
}
