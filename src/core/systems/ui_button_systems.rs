use log::{debug, info, warn};
use winit::window::CursorIcon;
use crate::core::components::color::Color;
use crate::core::components::{Hide, HidePropagated};
use crate::core::components::material::Material;
use crate::core::components::material::Material::Texture;
use crate::core::components::maths::hierarchy::{Children, Parent};
use crate::core::components::maths::transform::Transform;
use crate::core::components::ui::ui_button::UiButton;
use crate::core::components::ui::ui_image::UiImage;
use crate::core::components::ui::ui_text::UiText;
use crate::core::world::{GameData, World};

/// This system is responsible of handling components needed to represent buttons
/// It will detect and create needed components
pub(crate) fn set_childs_on_buttons(data: &mut GameData) {
    let mut to_add_entities = Vec::new();
    for (e, (ui_button, transform)) in data.query_mut::<(&UiButton, &Transform)>().without::<&Children>() {
        let mut ui_text = UiText::new(ui_button.text().to_string(), ui_button.font_ref());
        ui_text = ui_text.with_font_size(ui_button.font_size());
        if let Some(color) = ui_button.font_color() {
            ui_text = ui_text.with_font_color(color);
        }
        ui_text.set_padding(ui_button.padding());
        let mut material = Material::Color(Color::new(0, 0, 0, 0.));
        if let Some(color) = ui_button.background_color() {
            material = Material::Color(color);
        }
        to_add_entities.push((e, UiImage::new(ui_button.width() as f32, ui_button.height() as f32), material, ui_text, Transform::from_xyz(0., 0., transform.local_translation.z), Parent(e)));
    }

    to_add_entities.drain(0..).for_each(|(_, ui_image, texture, ui_text, transform, parent)| {
        data.push((ui_image, texture, ui_text, transform, parent));
    });
}

pub(crate) fn compute_hover(data: &mut GameData) {
    let (mx, my) = data.inputs().mouse_xy();
    let mut hover = false;
    let mut c = Vec::new();
    for (e, (ui_button, transform, children))
    in data.query_mut::<(&UiButton, &Transform, &mut Children)>()
        .without::<&Hide>()
        .without::<&HidePropagated>() {
        if transform.global_translation.x as f64 <= mx
            && (transform.global_translation.x + ui_button.width() as f32) as f64 >= mx
        && transform.global_translation.y as f64 <= my
            && (transform.global_translation.y + ui_button.height() as f32) as f64 >= my {
            c.push(*children.0.get(0).unwrap());
            hover = true;
        }
    }

    c.drain(0..).for_each(|e| {
        let e = data.entry_mut::<&mut Material>(e).expect("");
        *e = Material::Color(Color::new_rgb(124,56,90));
    } );

    if hover{
        data.window().set_cursor(CursorIcon::Pointer);
    } else{
        data.window().set_cursor(CursorIcon::Default);
    }
}