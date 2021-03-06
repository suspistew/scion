use std::io::{Error, Read, Write, ErrorKind};
use std::path::Path;
use std::fs::File;
use serde::{Serialize, Deserialize};

/// Main configuration used by `crate::Scion` to configure the game.
#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct ScionConfig{
    /// Name of the application, used for the window name in Windowed mode
    pub(crate) app_name: String
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
        }else{
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
    fn test_read_crystal_toml() {

        // Delete scion.toml before the test
        let path = Path::new("Scion.toml");
        std::fs::remove_file(path);

        let config = ScionConfigReader::read_or_create_scion_toml();
        assert!(config.is_ok());
    }
}

