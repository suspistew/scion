use legion::storage::Component;
use legion::world::SubWorld;
use legion::systems::CommandBuffer;
use legion::*;
use crate::core::resources::asset_manager::{AssetRef, AssetManager};
use crate::core::components::material::Material;

pub(crate) trait AssetResolverFn<T: Component>{
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
impl AssetResolverFn<Material> for MaterialAssetResolverFn{
    fn resolve(manager: &AssetManager, asset_ref: &AssetRef<Material>) -> Material {
        manager.get_material_for_ref(asset_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legion::{World, Resources, Schedule};
    use crate::core::resources::asset_manager::AssetManager;
    use crate::core::components::material::Material;
    use crate::core::systems::asset_ref_resolver_system::MaterialAssetResolverFn;
    use crate::core::components::color::Color;

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
        assert_eq!(true, world.entry(e).expect("").get_component::<Material>().is_err());

        schedule.execute(&mut world,&mut resources);

        assert_eq!(true, world.entry(e).expect("").get_component::<Material>().is_ok());

    }
}
