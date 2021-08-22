use legion::{World, Resources, Entity, IntoQuery};
use crate::core::legion_ext::ScionResourcesExtension;
use crate::core::resources::inputs::keycode::KeyCode;

pub fn try_debug(world: &mut World, resources: &mut Resources) {
    resources.inputs().keyboard().on_key_pressed(KeyCode::P, || {
        let mut query = <(Entity)>::query();
        let v: Vec<Entity> = query.iter(world).map(|e| *e).collect();
        v.iter().for_each(|e| {
            let entry = world.entry(*e).unwrap();
            println!("{:?}: {:?}", e, entry.archetype());
        }
        );
    });
}