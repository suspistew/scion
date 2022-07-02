use hecs::Component;
use crate::core::{
    components::material::Material,
    resources::asset_manager::{AssetManager, AssetRef},
};

pub(crate) trait AssetResolverFn<T: Component> {
    fn resolve(manager: &AssetManager, asset_ref: &AssetRef<T>) -> T;
}

/// System responsible to add an asset of type T to each entity with an assetRef<T>
pub(crate) fn asset_ref_resolver_system<T: Component, F: AssetResolverFn<T>>(world: &mut crate::core::world::World){
    let mut to_add = Vec::new();
    {
        let asset_manager = world.assets();
        for (e, asset_ref) in world.query::<&AssetRef<T>>().without::<&T>().iter() {
            to_add.push((e, (F::resolve(&asset_manager, asset_ref) )));
        }
    }
    to_add.drain(0..).for_each(|(e, a)|{
        let _r = world.add_components(e, (a,));
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

    use super::*;
    use crate::core::{
        components::{color::Color, material::Material},
        resources::asset_manager::AssetManager,
        systems::asset_ref_resolver_system::MaterialAssetResolverFn,
    };
    use crate::core::world::World;

    #[test]
    fn asset_ref_resolver_material_system_test() {
        let mut world = World::default();

        let mut manager = AssetManager::default();
        let asset_ref = manager.register_material(Material::Color(Color::new(1, 1, 1, 1.)));
        world.insert_resource(manager);

        let e = world.push((1, asset_ref.clone()));

        assert_eq!(true, world.entry::<&Material>(e).expect("").get().is_none());

        asset_ref_resolver_system::<Material, MaterialAssetResolverFn>(&mut world);

        assert_eq!(true, world.entry::<&Material>(e).expect("").get().is_some());
    }
}
