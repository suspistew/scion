use fern::Dispatch;
use log::LevelFilter;
use log::debug;
use std::io;

use serde::{Serialize, Deserialize};
use fern::colors::ColoredLevelConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct LoggerConfig {
    pub(crate) level_filter: LevelFilter,
}

impl Default for LoggerConfig{
    fn default() -> Self {
        Self {
            level_filter: LevelFilter::Info
        }
    }
}

/// Logging utility provided by Scion
pub(crate) struct Logger;

impl Logger {
    /// This will create and apply a logging dispatcher based upon fern.
    /// If None is provided, LoggerConfig::Default will be used.
    /// In every case, will be called at `Scion` app init, and this will try to
    /// apply a logging config. If one already exists, it won't replace it.
    pub fn init_logging(config: Option<LoggerConfig>) {
        let config = config.unwrap_or(LoggerConfig::default());
        let color_config = ColoredLevelConfig::new();
        fern::Dispatch::new().format(|out, message, record| {
            out.finish(format_args!(
                "[{level}][{target}] {message}",
                level = record.level(),
                target = record.target(),
                message = message,
            ))
        }).level(config.level_filter)
            .chain(
                fern::Dispatch::new()
                    .chain(io::stdout())
                    .format(move |out, message, record| {
                        let color = color_config.get_color(&record.level());
                        out.finish(format_args!(
                            "{color}{message}{color_reset}",
                            color = format!("\x1B[{}m", color.to_fg_str()),
                            message = message,
                            color_reset = "\x1B[0m",
                        ))
                    })
            )
            .apply()
            .unwrap_or_else(|_| {
                debug!("Logger can be set only ont time, skipping.")
            });
    }
}