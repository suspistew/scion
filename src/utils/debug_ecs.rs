use legion::{Entity, IntoQuery, Resources, World};

use crate::core::{legion_ext::ScionResourcesExtension};
use crate::core::resources::inputs::types::KeyCode;

pub fn try_debug(world: &mut World, resources: &mut Resources) {
    resources.inputs().on_key_pressed(KeyCode::P, || {
        let mut query = <Entity>::query();
        let v: Vec<Entity> = query.iter(world).map(|e| *e).collect();
        v.iter().for_each(|e| {
            let entry = world.entry(*e).unwrap();
            println!("{:?}: {:?}", e, entry.archetype());
        });
    });
}
