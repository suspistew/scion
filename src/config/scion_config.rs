use std::io::{Error, Read, Write, ErrorKind};
use std::path::Path;
use std::fs::File;
use serde::{Serialize, Deserialize};
use crate::config::window_config::WindowConfig;
use crate::utils::frame_limiter::{FrameLimiterStrategy, FrameLimiterConfig};
use crate::utils::logger::LoggerConfig;

/// Main configuration used by `crate::Scion` to configure the game.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ScionConfig {
    /// Name of the application
    pub(crate) app_name: String,
    /// Configuration for the game window
    pub(crate) window_config: Option<WindowConfig>,
    /// `FrameLimiterStrategy` to use while running the main loop. Will use Sleep with 60 fps by default
    pub(crate) frame_limiter: Option<FrameLimiterConfig>,
    /// Logger configuration to use.
    pub(crate) logger_config: Option<LoggerConfig>
}

impl Default for ScionConfig {
    fn default() -> Self {
        Self {
            app_name: "Scion game".to_string(),
            window_config: Some(Default::default()),
            frame_limiter: Some(Default::default()),
            logger_config: Some(Default::default())
        }
    }
}

pub struct ScionConfigReader;

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
        std::fs::remove_file(path);

        let config = ScionConfigReader::read_or_create_scion_toml();
        assert!(config.is_ok());
    }
}

