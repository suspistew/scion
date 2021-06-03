use legion::{storage::Component, systems::CommandBuffer, world::SubWorld, *};

use crate::core::{
    components::material::Material,
    resources::asset_manager::{AssetManager, AssetRef},
};

pub(crate) trait AssetResolverFn<T: Component> {
    fn resolve(manager: &AssetManager, asset_ref: &AssetRef<T>) -> T;
}

/// System responsible to add an asset of type T to each entity with an assetRef<T>
#[system]
#[read_component(AssetRef<T>)]
#[write_component(T)]
pub(crate) fn asset_ref_resolver<T: Component, F: AssetResolverFn<T>>(
    world: &mut SubWorld,
    cmd: &mut CommandBuffer,
    #[resource] asset_manager: &mut AssetManager,
) {
    <(Entity, &AssetRef<T>)>::query()
        .filter(!component::<T>())
        .for_each(world, |(e, asset_ref)| {
            cmd.add_component(*e, F::resolve(asset_manager, asset_ref));
        });
}

pub(crate) struct MaterialAssetResolverFn;
impl AssetResolverFn<Material> for MaterialAssetResolverFn {
    fn resolve(manager: &AssetManager, asset_ref: &AssetRef<Material>) -> Material {
        manager.get_material_for_ref(asset_ref)
    }
}

#[cfg(test)]
mod tests {
    use legion::{Resources, Schedule, World};

    use super::*;
    use crate::core::{
        components::{color::Color, material::Material},
        resources::asset_manager::AssetManager,
        systems::asset_ref_resolver_system::MaterialAssetResolverFn,
    };

    #[test]
    fn asset_ref_resolver_material_system_test() {
        let mut world = World::default();
        let mut resources = Resources::default();

        let mut manager = AssetManager::default();
        let asset_ref = manager.register_material(Material::Color(Color::new(1, 1, 1, 1.)));
        resources.insert(manager);
        let mut schedule = Schedule::builder()
            .add_system(asset_ref_resolver_system::<Material, MaterialAssetResolverFn>())
            .build();

        let e = world.push((1, asset_ref.clone()));
        assert_eq!(
            true,
            world
                .entry(e)
                .unwrap()
                .get_component::<Material>()
                .is_err()
        );

        schedule.execute(&mut world, &mut resources);

        assert_eq!(
            true,
            world
                .entry(e)
                .unwrap()
                .get_component::<Material>()
                .is_ok()
        );
    }
}
