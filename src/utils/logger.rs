use std::io;

use fern::colors::{Color, ColoredLevelConfig};
use log::debug;

use crate::config::logger_config::LoggerConfig;

/// Logging utility provided by Scion
pub(crate) struct Logger;

impl Logger {
    /// This will create and apply a logging dispatcher based upon fern.
    /// If None is provided, LoggerConfig::Default will be used.
    /// In every case, will be called at `Scion` app init, and this will try to
    /// apply a logging config. If one already exists, it won't replace it.
    pub fn init_logging(config: Option<LoggerConfig>) {
        let config = config.unwrap_or_default();
        let color_config = ColoredLevelConfig {
            error: Color::Red,
            warn: Color::Yellow,
            info: Color::Blue,
            debug: Color::White,
            trace: Color::BrightWhite,
        };
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}] : \n    -> {}", record.level(), record.target(), message
                ))
            })
            .level(config.level_filter)
            .chain(
                fern::Dispatch::new()
                    .chain(io::stdout())
                    .level(config.level_filter)
                    .level_for("scion", config.scion_level_filter)
                    .level_for("wgpu_core", log::LevelFilter::Off)
                    .level_for("wgpu_hal", log::LevelFilter::Off)
                    .level_for("naga", log::LevelFilter::Off)
                    .format(move |out, message, record| {
                        let color = color_config.get_color(&record.level());
                        out.finish(format_args!(
                            "{color}{message}{color_reset}",
                            color = format!("\x1B[{}m", color.to_fg_str()),
                            message = message,
                            color_reset = "\x1B[0m",
                        ))
                    }))
            .apply()
            .unwrap_or_else(|_| debug!("Logger can be set only one time, skipping."));
    }
}
