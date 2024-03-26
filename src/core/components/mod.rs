//! Contains all the components provided by `Scion`

pub use shapes::{square::Square, triangle::Triangle};

pub mod animations;
pub mod color;
pub mod material;
pub mod maths;
pub mod shapes;
pub mod tiles;
pub mod ui;

/// Struct to add to any entity to 'hide' it during renderig
pub struct Hide;

pub(crate) struct HidePropagated;
