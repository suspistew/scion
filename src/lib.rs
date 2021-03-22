//! This is a library to create apps & games using wgpu / winit / legion
//!
//! If it's missing something that you need (and sure it will), create an issue on [GitHub issue tracker](https://github.com/grzi/scion/issues)
//!
//! # Building a `scion` app
//!
//! To create a  [`Scion`] application, you either need to call the [`Scion::app()`], [`Scion::app_with_config()`] or [`Scion::app_with_config_path()`] function.
//! This will give you access to the [`ScionBuilder`] where you can add [legion](https://docs.rs/legion/0.4.0/legion/) systems, [`rendering::RendererType`], [`game_layer::GameLayer`].
//!
//! # Example
//!
//! ```no_run
//! use scion::{legion::system, Scion};
//!
//! #[system]
//! fn hello() {
//!     println!("Hello world from a system");
//! }
//!
//! fn main() {
//!     Scion::app().with_system(hello_system()).run();
//! }
//! ```

// Convenience reexport
// Convenience uses
pub use application::{Scion, ScionBuilder};
pub use legion;
pub use ultraviolet;

// internal export
mod application;
pub mod config;
pub mod game_layer;
pub mod inputs;
pub mod rendering;
pub mod utils;
pub mod state;
