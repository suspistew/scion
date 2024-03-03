use crate::core::components::maths::{coordinates::Coordinates, transform::Transform};
use crate::utils::maths::Vector;
use hecs::Entity;
use log::info;

struct RectangleColliderInfo<'a> {
    width: &'a usize,
    height: &'a usize,
    transform: &'a Transform,
    offset: &'a Vector,
}

impl<'a> RectangleColliderInfo<'a> {
    fn of(
        width: &'a usize,
        height: &'a usize,
        transform: &'a Transform,
        offset: &'a Vector,
    ) -> Self {
        Self { width, height, transform, offset }
    }
}

/// `ColliderMask` will serve as a 'mask' to allow filter while collisions happen
#[derive(PartialEq, Clone, Eq, Hash, Debug)]
pub enum ColliderMask {
    None,
    Character,
    Bullet,
    Death,
    Landscape,
    Custom(String),
}

/// `ColliderType` will determine the shape of the collider.
#[derive(Clone)]
pub enum ColliderType {
    Square(usize),
    Rectangle(usize, usize),
}

/// The main collider representation to add to an entity, using the new function
#[derive(Clone)]
pub struct Collider {
    collider_mask: ColliderMask,
    collider_type: ColliderType,
    collision_filter: Vec<ColliderMask>,
    collisions: Vec<Collision>,
    offset: Vector,
    debug_lines: bool,
}

impl Collider {
    /// Creates a new collider. Note that an empty collision_filter means that this colliders
    /// won't collide
    pub fn new(
        collider_mask: ColliderMask,
        collision_filter: Vec<ColliderMask>,
        collider_type: ColliderType,
    ) -> Self {
        Collider {
            collider_mask,
            collider_type,
            collision_filter,
            collisions: vec![],
            offset: Vector::default(),
            debug_lines: false,
        }
    }

    pub fn with_debug_lines(mut self) -> Self {
        self.debug_lines = true;
        self
    }

    pub fn with_offset(mut self, offset: Vector) -> Self {
        self.offset = offset;
        self
    }

    /// Return whether or not this collider colliding to any other collider ?
    pub fn is_colliding(&self) -> bool {
        !self.collisions.is_empty()
    }

    /// Returns an iterator of current collisions
    pub fn collisions(&self) -> &Vec<Collision> {
        &self.collisions
    }

    /// The mask of this collider
    pub fn mask(&self) -> &ColliderMask {
        &self.collider_mask
    }

    /// The mask of this collider
    pub fn mask_cloned(&self) -> ColliderMask {
        self.collider_mask.clone()
    }

    /// The filters of this collider
    pub fn filters(&self) -> &Vec<ColliderMask> {
        &self.collision_filter
    }

    /// Retrieve the collider type of this collider
    pub fn collider_type(&self) -> &ColliderType {
        &self.collider_type
    }

    /// Retrieve the offset of this collider
    pub fn offset(&self) -> &Vector {
        &self.offset
    }

    pub(crate) fn debug_lines(&self) -> bool {
        self.debug_lines
    }

    pub(crate) fn clear_collisions(&mut self) {
        self.collisions.clear();
    }

    pub(crate) fn can_collide_with(&self, other: &Collider) -> bool {
        self.collision_filter.is_empty() || self.collision_filter.contains(&other.collider_mask)
    }

    pub(crate) fn collides_with(
        &self,
        self_transform: &Transform,
        target_collider: &Collider,
        target_transform: &Transform,
    ) -> Option<CollisionArea> {
        if !self.can_collide_with(target_collider) {
            return None;
        }
        match (&self.collider_type, &target_collider.collider_type) {
            (ColliderType::Square(self_size), ColliderType::Square(target_size)) => {
                rectangle_collider_vs_rectangle_collider(
                    RectangleColliderInfo::of(
                        self_size,
                        self_size,
                        self_transform,
                        &self.offset,
                    ),
                    RectangleColliderInfo::of(
                        target_size,
                        target_size,
                        target_transform,
                        &target_collider.offset,
                    ),
                )
            }
            (
                ColliderType::Rectangle(self_width, self_height),
                ColliderType::Rectangle(target_width, target_height),
            ) => rectangle_collider_vs_rectangle_collider(
                RectangleColliderInfo::of(
                    self_width,
                    self_height,
                    self_transform,
                    &self.offset,
                ),
                RectangleColliderInfo::of(
                    target_width,
                    target_height,
                    target_transform,
                    &target_collider.offset,
                ),
            ),
            (
                ColliderType::Square(self_size),
                ColliderType::Rectangle(target_width, target_height),
            ) => rectangle_collider_vs_rectangle_collider(
                RectangleColliderInfo::of(self_size, self_size, self_transform, &self.offset),
                RectangleColliderInfo::of(
                    target_width,
                    target_height,
                    target_transform,
                    &target_collider.offset,
                ),
            ),
            (
                ColliderType::Rectangle(self_width, self_height),
                ColliderType::Square(target_size),
            ) => rectangle_collider_vs_rectangle_collider(
                RectangleColliderInfo::of(
                    self_width,
                    self_height,
                    self_transform,
                    &self.offset,
                ),
                RectangleColliderInfo::of(
                    target_size,
                    target_size,
                    target_transform,
                    &target_collider.offset,
                ),
            ),
        }
    }

    pub(crate) fn add_collisions(&mut self, collisions: &mut Vec<Collision>) {
        self.collisions.append(collisions);
    }
}

fn rectangle_collider_vs_rectangle_collider(
    self_collider: RectangleColliderInfo,
    target_collider: RectangleColliderInfo,
) -> Option<CollisionArea> {
    let (x1, y1) = (self_collider.transform.global_translation.x + self_collider.offset.x,
                    self_collider.transform.global_translation.y + self_collider.offset.y + (*self_collider.height as f32));
    let (x2, y2) = (self_collider.transform.global_translation.x + self_collider.offset.x + (*self_collider.width as f32),
                    self_collider.transform.global_translation.y + self_collider.offset.y);
    let (x3, y3) = (target_collider.transform.global_translation.x + target_collider.offset.x,
                    target_collider.transform.global_translation.y + target_collider.offset.y + (*target_collider.height as f32));
    let (x4, y4) = (target_collider.transform.global_translation.x + target_collider.offset.x + (*target_collider.width as f32),
                    target_collider.transform.global_translation.y + target_collider.offset.y);

    let x5 = x1.max(x3);
    let x6 = x2.min(x4);
    let y5 = y1.min(y3);
    let y6 = y2.max(y4);

    if x5 > x6 || y6 > y5 {
        None
    } else {
        Some(CollisionArea { start_point: Coordinates::new(x5, y6), end_point: Coordinates::new(x6, y5) })
    }
}

/// Representation of a collision
#[derive(Clone, Debug)]
pub struct Collision {
    pub(crate) mask: ColliderMask,
    pub(crate) entity: Entity,
    pub(crate) coordinates: Coordinates,
    pub(crate) collision_area: CollisionArea,
}

impl Collision {
    pub fn entity(&self) -> &Entity {
        &self.entity
    }
    pub fn mask(&self) -> &ColliderMask {
        &self.mask
    }
    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
    pub fn area(&self) -> &CollisionArea {
        &self.collision_area
    }
}

#[derive(Clone, Debug)]
pub struct CollisionArea {
    pub(crate) start_point: Coordinates,
    pub(crate) end_point: Coordinates,
}

impl CollisionArea{
    pub fn start_point(&self) -> &Coordinates{
        &self.start_point
    }

    pub fn end_point(&self) -> &Coordinates{
        &self.end_point
    }
}

/// Internal component used to keep track of a collider debug display
pub(crate) struct ColliderDebug;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_collide_with() {
        let bullet = Collider::new(ColliderMask::Bullet, vec![], ColliderType::Square(5));
        let ship = Collider::new(
            ColliderMask::Character,
            vec![ColliderMask::Bullet],
            ColliderType::Square(5),
        );
        let land = Collider::new(ColliderMask::Landscape, vec![], ColliderType::Square(5));

        assert_eq!(false, ship.can_collide_with(&land));
        assert_eq!(true, ship.can_collide_with(&bullet));
    }

    #[test]
    fn test_collides_with_square() {
        let bullet = Collider::new(ColliderMask::Bullet, vec![], ColliderType::Square(5));
        let ship = Collider::new(ColliderMask::Character, vec![], ColliderType::Square(5));

        let bullet_transform = Transform::from_xy(4., 4.);
        let bullet_transform2 = Transform::from_xy(9., 9.);
        let ship_transform_in = Transform::from_xy(5., 5.);
        let ship_transform_in2 = Transform::from_xy(9., 9.);
        let ship_transform_out = Transform::from_xy(50., 50.);

        assert_eq!(true, ship.collides_with(&ship_transform_in, &bullet, &bullet_transform).is_some());
        assert_eq!(true, ship.collides_with(&ship_transform_in2, &bullet, &bullet_transform).is_some());
        assert_eq!(true, ship.collides_with(&ship_transform_in, &bullet, &bullet_transform2).is_some());
        assert_eq!(false, bullet.collides_with(&ship_transform_out, &bullet, &bullet_transform).is_some());
    }

    #[test]
    fn test_does_notcollides_with_square_if_offsets_too_far() {
        let mut bullet = Collider::new(
            ColliderMask::Bullet,
            vec![ColliderMask::Character],
            ColliderType::Square(5),
        );
        bullet = bullet.with_offset(Vector::new(-3., -3.));

        let mut ship = Collider::new(
            ColliderMask::Character,
            vec![ColliderMask::Bullet],
            ColliderType::Square(5),
        );
        ship = ship.with_offset(Vector::new(3., 3.));

        let bullet_transform = Transform::from_xy(5., 5.);
        let ship_transform = Transform::from_xy(5., 5.);

        assert_eq!(false, bullet.collides_with(&bullet_transform, &ship, &ship_transform).is_some());
    }

    #[test]
    fn test_does_collides_with_square_if_offsets_close_enough() {
        let mut bullet = Collider::new(
            ColliderMask::Bullet,
            vec![ColliderMask::Character],
            ColliderType::Square(5),
        );
        bullet = bullet.with_offset(Vector::new(-1., -1.));

        let mut ship = Collider::new(
            ColliderMask::Character,
            vec![ColliderMask::Bullet],
            ColliderType::Square(5),
        );
        ship = ship.with_offset(Vector::new(1., 1.));

        let bullet_transform = Transform::from_xy(5., 5.);
        let ship_transform = Transform::from_xy(5., 5.);

        assert_eq!(true, bullet.collides_with(&bullet_transform, &ship, &ship_transform).is_some());
    }
}
