use std::slice::Iter;

use legion::Entity;

use crate::core::components::maths::transform::{Transform};
use crate::core::components::maths::coordinates::Coordinates;

struct SquareColliderInfo<'a> {
    size: &'a usize,
    transform: &'a Transform,
}

impl<'a> SquareColliderInfo<'a> {
    fn of(size: &'a usize, transform: &'a Transform) -> Self {
        Self { size, transform }
    }
}

/// `ColliderMask` will serve as a 'mask' to allow filter while collisions happen
#[derive(PartialEq, Clone)]
pub enum ColliderMask {
    None,
    Character,
    Bullet,
    Death,
    Landscape,
    Custom(String),
}

/// `ColliderType` will determine the shape of the collider.
pub enum ColliderType {
    Square(usize),
}

/// The main collider representation to add to an entity, using the new function
pub struct Collider {
    collider_mask: ColliderMask,
    collider_type: ColliderType,
    collision_filter: Vec<ColliderMask>,
    collisions: Vec<Collision>,
}

impl Collider {
    /// Creates a new collider. Note that an empty collision_filter means that this colliders
    /// collides with every other colliders
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
        }
    }

    /// Return whether or not this collider colliding to any other collider ?
    pub fn is_colliding(&self) -> bool {
        !self.collisions.is_empty()
    }

    pub fn collisions(&self) -> Iter<'_, Collision> {
        self.collisions.iter()
    }
    pub fn mask(&self) -> &ColliderMask {
        &self.collider_mask
    }
    pub(crate) fn passive(&self) -> bool {
        self.collision_filter.len() == 1 && self.collision_filter.contains(&ColliderMask::None)
    }

    pub(crate) fn clear_collisions(&mut self) {
        self.collisions.clear();
    }

    pub(crate) fn can_collide_with(&self, other: &Collider) -> bool {
        return self.collision_filter.is_empty()
            || self.collision_filter.contains(&other.collider_mask);
    }

    pub(crate) fn collides_with(
        &self,
        self_transform: &Transform,
        target_collider: &Collider,
        target_transform: &Transform,
    ) -> bool {
        self.can_collide_with(target_collider)
            && match (&self.collider_type, &target_collider.collider_type) {
                (ColliderType::Square(self_size), ColliderType::Square(target_size)) => {
                    square_collider_vs_square_collider(
                        SquareColliderInfo::of(self_size, self_transform),
                        SquareColliderInfo::of(target_size, target_transform),
                    )
                }
            }
    }

    pub(crate) fn add_collisions(&mut self, collisions: &mut Vec<Collision>) {
        self.collisions.append(collisions);
    }
}

fn square_collider_vs_square_collider(
    self_collider: SquareColliderInfo,
    target_collider: SquareColliderInfo,
) -> bool {
    let p1 = self_collider.transform.global_translation;
    let p2 = target_collider.transform.global_translation;
    let (x_min_p1, x_max_p1, y_min_p1, y_max_p1, x_min_p2, x_max_p2, y_min_p2, y_max_p2) = (
        p1.x(),
        p1.x() + *self_collider.size as f32,
        p1.y(),
        p1.y() + *self_collider.size as f32,
        p2.x(),
        p2.x() + *target_collider.size as f32,
        p2.y(),
        p2.y() + *target_collider.size as f32,
    );
    x_min_p1 < x_max_p2 && x_max_p1 > x_min_p2 && y_min_p1 < y_max_p2 && y_max_p1 > y_min_p2
}

/// Representation of a collision
pub struct Collision {
    pub(crate) mask: ColliderMask,
    pub(crate) entity: Entity,
    pub(crate) coordinates: Coordinates,
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
}

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
        let _ship = Collider::new(ColliderMask::Character, vec![], ColliderType::Square(5));

        let bullet_transform = Transform::from_xy(4., 4.);
        let ship_transform_in = Transform::from_xy(5., 5.);
        let ship_transform_out = Transform::from_xy(50., 50.);

        assert_eq!(
            true,
            bullet.collides_with(&ship_transform_in, &bullet, &bullet_transform)
        );
        assert_eq!(
            false,
            bullet.collides_with(&ship_transform_out, &bullet, &bullet_transform)
        );
    }
}
