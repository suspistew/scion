use hecs::Entity;
use std::collections::HashSet;

use crate::core::components::{
    maths::{coordinates::Coordinates, hierarchy::Parent, transform::Transform},
    ui::{
        font::Font,
        ui_image::UiImage,
        ui_text::{UiText, UiTextImage},
        UiComponent,
    },
};
use crate::core::world::{GameData, World};

pub(crate) fn sync_text_value_system(data: &mut GameData) {
    let (world, resources) = data.split();
    for(_e, ui_text) in world.query_mut::<&mut UiText>(){
        if let Some(function) = ui_text.sync_fn {
            ui_text.set_text(function(resources));
        }
    }
}

pub(crate) fn ui_text_bitmap_update_system(data: &mut GameData) {
    let mut parent_to_remove: HashSet<Entity> = HashSet::new();
    let mut to_add: Vec<(UiTextImage, UiComponent, Transform, Parent)> = Vec::new();
    let (world, resources) = data.split();

    for (e, (ui_text, transform)) in world.query_mut::<(&mut UiText, &Transform)>() {
        if ui_text.dirty {
            parent_to_remove.insert(e);
            let font = resources.assets_mut().get_font_for_ref(ui_text.font_ref());
            let Font::Bitmap { texture_path, chars, width, height, texture_columns, texture_lines } =
                font;
            let texture_width = texture_columns * width;
            let texture_height = texture_lines * height;

            for (index, character) in ui_text.text().chars().enumerate() {
                let (line, column) =
                    Font::find_line_and_column(&chars, texture_columns, character);

                let uvs = [
                    Coordinates::new(
                        (column * width) / texture_width,
                        (line * height) / texture_height,
                    ),
                    Coordinates::new(
                        (column * width) / texture_width,
                        (line * height + height) / texture_height,
                    ),
                    Coordinates::new(
                        (column * width + width) / texture_width,
                        (line * height + height) / texture_height,
                    ),
                    Coordinates::new(
                        (column * width + width) / texture_width,
                        (line * height) / texture_height,
                    ),
                ];

                let mut char_transform = Transform::from_xy(index as f32 * (width + 1.), 0.);
                char_transform.set_z(transform.translation().z());
                to_add.push((
                    UiTextImage(UiImage::new_with_uv_map(
                        width as f32,
                        height as f32,
                        texture_path.clone(),
                        uvs,
                    )),
                    UiComponent,
                    char_transform,
                    Parent(e),
                ));
            }
            ui_text.dirty = false;
        }
    }

    let entities_to_remove = data
        .query::<(&UiTextImage, &Parent)>()
        .iter()
        .filter(|(_e, (_, p))| parent_to_remove.contains(&p.0))
        .map(|(e, _)| e)
        .collect::<Vec<_>>();

    entities_to_remove.iter().for_each(|e| {
        let _r = data.remove(*e);
    });

    to_add.drain(0..).for_each(|c| {
        data.push(c);
    });
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::components::{
        maths::transform::Transform,
        ui::{
            font::Font,
            ui_text::{UiText, UiTextImage},
        },
    };
    use crate::core::resources::asset_manager::{AssetManager, AssetRef};
    use crate::core::world::World;

    fn get_test_ui_text(assets: &mut AssetManager) -> UiText {
        // First we add an UiText to the world
        let font = Font::Bitmap {
            texture_path: "test".to_string(),
            chars: "abcdefg".to_string(),
            texture_columns: 7.,
            texture_lines: 1.,
            width: 5.,
            height: 5.,
        };

        let asset = assets.register_font(font);

        UiText::new("abf".to_string(), asset)
    }

    #[test]
    fn ui_text_without_transform_should_not_generate_ui_image() {
        let mut world = GameData::default();
        let mut manager = AssetManager::default();
        let _entity = world.push((get_test_ui_text(&mut manager),));
        world.insert_resource(manager);

        ui_text_bitmap_update_system(&mut world);

        let cpt = world.query::<&UiTextImage>().iter().count();
        assert_eq!(0, cpt);
    }

    #[test]
    fn ui_text_with_transform_should_generate_ui_image() {
        let mut world = GameData::default();

        let mut manager = AssetManager::default();
        let _entity = world.push((get_test_ui_text(&mut manager), Transform::default()));
        world.insert_resource(manager);

        ui_text_bitmap_update_system(&mut world);

        let cpt = world.query::<&UiTextImage>().iter().count();
        assert_eq!(3, cpt);
    }

    struct Test{pub score: usize}

    #[test]
    fn ui_text_synchronized() {
        let mut world = GameData::default();
        world.insert_resource(Test{ score: 5 });

        let mut manager = AssetManager::default();
        let text_synced = get_test_ui_text(&mut manager)
            .sync_value(|g| g.get_resource::<Test>().unwrap().score.to_string());
        let _entity = world.push(( text_synced, Transform::default()));
        world.insert_resource(manager);

        let txt = world.query::<&UiText>().iter().next().unwrap().1.text().to_string();
        assert_eq!("abf".to_string(), txt);

        sync_text_value_system(&mut world);

        let txt = world.query::<&UiText>().iter().next().unwrap().1.text().to_string();
        assert_eq!("5".to_string(), txt);
    }
}
