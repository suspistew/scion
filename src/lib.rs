//! This is a library to create apps & games using wgpu / winit / legion
//!
//! If it's missing something that you need (and sure it will), create an issue on [GitHub issue tracker](https://github.com/grzi/scion/issues)
//!
//! ```

// Convenience reexport
// Convenience uses
pub use application::{Scion, ScionBuilder};

pub use ultraviolet;
pub use hecs::Entity;
pub use winit::window::CursorIcon;
pub use log;

// internal export
mod application;
pub mod config;
pub mod core;
pub mod graphics;
pub mod utils;
