use scion::core::components::{
    material::Material,
    maths::{
        collider::{Collider, ColliderMask},
        transform::Transform,
    },
};
use scion::core::world::{GameData, World};

use crate::Hero;

pub(crate) fn collider_system(data: &mut GameData) {
    for (_, (collider, hero, material, transform)) in
        data.query_mut::<(&mut Collider, &mut Hero, &mut Material, &Transform)>()
    {
        if let Material::Color(_c) = material {
            collider.collisions().iter().for_each(|collision| match collision.mask() {
                ColliderMask::Death => std::process::exit(0),
                ColliderMask::Landscape => {
                    if collision.coordinates().y() > transform.global_translation().y() {
                        hero.landed = true;
                    }
                }
                _ => {}
            });
        }
    }
}
