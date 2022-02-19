use log::LevelFilter;
use serde::{Deserialize, Serialize};

/// Logger configuration used by Scion.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggerConfig {
    pub level_filter: LevelFilter,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self { level_filter: LevelFilter::Warn }
    }
}
