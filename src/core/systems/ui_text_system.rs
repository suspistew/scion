use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, Query};

use crate::core::components::{
    maths::{
        hierarchy::Parent,
        transform::{Coordinates, Transform},
    },
    ui::{
        font::Font,
        ui_image::UiImage,
        ui_text::{UiText, UiTextImage},
        UiComponent,
    },
};

/// System responsible to create/delete/update the entities linked to any ui_text with a bitmap font
#[system]
pub(crate) fn ui_text_bitmap_update(
    world: &mut SubWorld,
    cmd: &mut CommandBuffer,
    query_ui_texts: &mut Query<(Entity, &mut UiText, &Transform)>,
    query_ui_text_images: &mut Query<(Entity, &UiTextImage, &Parent)>,
) {
    let (mut world_1, world_2) = world.split_for_query(query_ui_texts);
    query_ui_texts
        .iter_mut(&mut world_1)
        .for_each(|(entity, ui_text, transform)| {
            if ui_text.dirty {
                if let Font::Bitmap {
                    texture_path,
                    chars,
                    width,
                    height,
                    texture_columns,
                    texture_lines,
                } = ui_text.font()
                {
                    let texture_width = texture_columns * width;
                    let texture_height = texture_lines * height;

                    query_ui_text_images
                        .iter(&world_2)
                        .filter(|(_, _, parent)| parent.0 == *entity)
                        .for_each(|(e, _, _)| cmd.remove(*e));
                    for (index, character) in ui_text.text().chars().enumerate() {
                        let (line, column) =
                            Font::find_line_and_column(&chars, *texture_columns, character);

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

                        let mut char_transform = Transform::new(
                            Coordinates::new(
                                transform.translation().x() + (index as f32 * (width + 1.)),
                                transform.translation().y(),
                            ),
                            1.0,
                            0.,
                        );
                        char_transform.set_layer(transform.translation().layer());
                        cmd.push((
                            UiTextImage(UiImage::new_with_uv_map(
                                *width as f32,
                                *height as f32,
                                texture_path.clone(),
                                uvs,
                            )),
                            UiComponent,
                            char_transform,
                            Parent(*entity),
                        ));
                    }
                }
                ui_text.dirty = false;
            }
        });
}

#[cfg(test)]
mod tests {
    use legion::{Entity, IntoQuery, Resources, Schedule, World};

    use super::*;
    use crate::core::components::{
        maths::transform::Transform,
        ui::{
            font::Font,
            ui_text::{UiText, UiTextImage},
        },
    };

    fn get_test_ui_text() -> UiText {
        // First we add an UiText to the world
        let font = Font::Bitmap {
            texture_path: "test".to_string(),
            chars: "abcdefg".to_string(),
            texture_columns: 7.,
            texture_lines: 1.,
            width: 5.,
            height: 5.,
        };

        UiText::new("abf".to_string(), font)
    }

    #[test]
    fn ui_text_without_transform_should_not_generate_ui_image() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule = Schedule::builder()
            .add_system(ui_text_bitmap_update_system())
            .build();

        let _entity = world.push((get_test_ui_text(),));
        schedule.execute(&mut world, &mut resources);
        let vec: Vec<(&Entity, &UiTextImage)> =
            <(Entity, &UiTextImage)>::query().iter(&world).collect();
        assert_eq!(0, vec.len());
    }

    #[test]
    fn ui_text_with_transform_should_generate_ui_image() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule = Schedule::builder()
            .add_system(ui_text_bitmap_update_system())
            .build();

        let _entity = world.push((get_test_ui_text(), Transform::default()));
        schedule.execute(&mut world, &mut resources);
        let vec: Vec<(&Entity, &UiTextImage)> =
            <(Entity, &UiTextImage)>::query().iter(&world).collect();
        assert_eq!(3, vec.len());
    }
}
