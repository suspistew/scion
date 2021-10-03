pub mod camera;
pub mod collider;
pub mod coordinates;
pub mod hierarchy;
pub mod transform;

/// `Pivot` tells where the pivot point of a component is
pub enum Pivot {
    /// Pivot is on the top left corner of the shape
    TopLeft,
    /// Pivot is on the center of the shape
    Center,
}
