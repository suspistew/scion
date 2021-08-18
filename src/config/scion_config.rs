use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::config::{logger_config::LoggerConfig, window_config::WindowConfig};

/// Main configuration used by `crate::Scion` to configure the game.
/// Please use [`ScionConfigBuilder`] if you want to build if from code.
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

/// `ScionConfigBuilder` is a convenience builder to create a `ScionConfig` from code.
pub struct ScionConfigBuilder {
    config: ScionConfig,
}

impl ScionConfigBuilder {
    /// Create a new `ScionConfigBuilder` builder
    pub fn new() -> Self { Self { config: Default::default() } }

    /// Sets the app name for scion. Will also be used for the window name
    pub fn with_app_name(mut self, app_name: String) -> Self {
        self.config.app_name = app_name;
        self
    }

    /// Sets the logger configuration for the application
    pub fn with_logger_config(mut self, logger_config: LoggerConfig) -> Self {
        self.config.logger_config = Some(logger_config);
        self
    }

    /// Sets the main window configuration. `WindowConfig` can be built using `WindowConfigBuilder`
    pub fn with_window_config(mut self, window_config: WindowConfig) -> Self {
        self.config.window_config = Some(window_config);
        self
    }

    /// Retrieves the configuration built
    pub fn get(self) -> ScionConfig { self.config }
}

pub(crate) struct ScionConfigReader;

impl ScionConfigReader {
    pub(crate) fn read_or_create_default_scion_json() -> Result<ScionConfig, Error> {
        let path = Path::new("scion.json");
        let path_exists = path.exists();

        if !path_exists {
            println!("Couldn't find `scion.json` configuration file. Generating a new one");
            let config = ScionConfig::default();
            let mut file = File::create(path)?;
            file.write_all(serde_json::to_vec(&config).unwrap().as_slice())?;
            Ok(config)
        } else {
            ScionConfigReader::read_scion_config(path)
        }
    }

    pub(crate) fn read_scion_json(path: &Path) -> Result<ScionConfig, Error> {
        let path_exists = path.exists();
        if !path_exists {
            return Err(Error::new(ErrorKind::NotFound, "File not found"));
        }
        ScionConfigReader::read_scion_config(path)
    }

    fn read_scion_config(path: &Path) -> Result<ScionConfig, Error> {
        let mut scion_config = File::open(path)?;
        let mut bytes = Vec::new();
        scion_config.read_to_end(&mut bytes)?;
        let config = serde_json::from_slice(bytes.as_slice())?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_scion_json() {
        // Delete scion.json before the test
        let path = Path::new("scion.json");
        let _r = std::fs::remove_file(path);

        let config = ScionConfigReader::read_or_create_default_scion_json();
        assert!(config.is_ok());
    }
}
