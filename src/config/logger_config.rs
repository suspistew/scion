use log::LevelFilter;
use serde::{Deserialize, Serialize};

/// Logger configuration used by Scion.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggerConfig {
    pub scion_level_filter: LevelFilter,
    pub level_filter: LevelFilter,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self { scion_level_filter: LevelFilter::Info, level_filter: LevelFilter::Info }
    }
}
