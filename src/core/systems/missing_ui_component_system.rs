use crate::core::components::ui::UiComponent;
use crate::core::world::{GameData, World};
use hecs::Component;

/// System responsible to add the UiComponent to any T missing its uiComponent
pub(crate) fn missing_ui_component_system<T: Component>(data: &mut GameData) {
    let mut to_add = Vec::new();
    {
        for (e, _) in data.query::<&T>().without::<&UiComponent>().iter() {
            to_add.push(e);
        }
    }
    to_add.drain(0..).for_each(|e| {
        let _r = data.add_components(e, (UiComponent,));
    });
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::components::ui::{ui_image::UiImage, UiComponent};
    use crate::core::world::World;

    #[test]
    fn missing_ui_comp_system_test() {
        let mut world = GameData::default();

        let e = world.push((UiImage::new(1., 1., "".to_string()),));

        assert_eq!(true, world.entry::<&UiComponent>(e).expect("").get().is_none());

        missing_ui_component_system::<UiImage>(&mut world);

        assert_eq!(true, world.entry::<&UiComponent>(e).expect("").get().is_some());
    }
}
