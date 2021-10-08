use legion::{IntoQuery, Resources, World};
use scion::core::{
    components::{
        color::Color,
        material::Material,
        maths::{
            collider::{Collider, ColliderMask, ColliderType},
            transform::Transform,
        },
        shapes::rectangle::Rectangle,
    },
    game_layer::SimpleGameLayer,
    legion_ext::ScionResourcesExtension,
};
use winit::window::CursorIcon;

use crate::main_layer::{CursorState, LineDirection};

pub struct Line {
    pub direction: LineDirection,
}

pub struct InputLayer {
    mouse_state: CursorState,
    state_before_border: Option<CursorState>,
}

impl Default for InputLayer {
    fn default() -> Self { Self { mouse_state: CursorState::ROW, state_before_border: None } }
}

impl SimpleGameLayer for InputLayer {
    fn update(&mut self, world: &mut World, resources: &mut Resources) {
        let mut change_mouse_state = false;
        let mouse_pos = resources.inputs().mouse_xy();
        if !self.compute_mouse_on_border(mouse_pos, resources) {
            resources
                .inputs()
                .on_right_click_released(|_pos_x, _pos_y| change_mouse_state = true);
            if change_mouse_state {
                let new_icon = self.compute_new_icon(resources);
                resources.window().set_cursor(new_icon);
            } else {
                if self.mouse_state != CursorState::ROW && self.mouse_state != CursorState::COLUMN {
                    self.mouse_state = self.state_before_border.as_ref().unwrap().clone();
                    self.state_before_border = None;
                    self.change_cursor(resources);
                }
            }
        }
        resources.inputs().on_left_click_pressed(|pos_x, pos_y| {
            let len = <(&Rectangle,)>::query().iter(world).collect::<Vec<(&Rectangle,)>>().len();

            if len == 0 && pos_x > 10. && pos_y > 10. {
                let x = (pos_x as usize - 10) / 16;
                let y = (pos_y as usize - 10) / 16;
                if self.mouse_state == CursorState::ROW {
                    world.push((
                        Transform::from_xyz(x as f32 * 16. + 10., y as f32 * 16. + 10., 20),
                        Rectangle::new(16., 16., None),
                        Material::Color(Color::new(255, 0, 0, 0.3)),
                        Line { direction: LineDirection::TOP },
                        Collider::new(
                            ColliderMask::Landscape,
                            vec![
                                ColliderMask::Bullet,
                                ColliderMask::Custom("BORDER_CUSTOM_HORIZONTAL".to_string()),
                                ColliderMask::Custom("BORDER_CUSTOM_VERTICAL".to_string()),
                                ColliderMask::Custom("BORDER_TOP".to_string()),
                                ColliderMask::Custom("BORDER_BOTTOM".to_string()),
                                ColliderMask::Custom("BORDER_LEFT".to_string()),
                                ColliderMask::Custom("BORDER_RIGHT".to_string()),
                            ],
                            ColliderType::Rectangle(16, 16),
                        ),
                    ));
                    world.push((
                        Transform::from_xyz(x as f32 * 16. + 10., y as f32 * 16. + 26., 20),
                        Rectangle::new(16., 16., None),
                        Material::Color(Color::new(0, 0, 255, 0.3)),
                        Line { direction: LineDirection::BOTTOM },
                        Collider::new(
                            ColliderMask::Landscape,
                            vec![
                                ColliderMask::Bullet,
                                ColliderMask::Custom("BORDER_CUSTOM".to_string()),
                                ColliderMask::Custom("BORDER_TOP".to_string()),
                                ColliderMask::Custom("BORDER_BOTTOM".to_string()),
                                ColliderMask::Custom("BORDER_LEFT".to_string()),
                                ColliderMask::Custom("BORDER_RIGHT".to_string()),
                            ],
                            ColliderType::Rectangle(16, 16),
                        ),
                    ));
                } else if self.mouse_state == CursorState::COLUMN {
                    world.push((
                        Transform::from_xyz(x as f32 * 16. + 10., y as f32 * 16. + 10., 20),
                        Rectangle::new(16., 16., None),
                        Material::Color(Color::new(255, 0, 0, 0.3)),
                        Line { direction: LineDirection::LEFT },
                        Collider::new(
                            ColliderMask::Landscape,
                            vec![
                                ColliderMask::Bullet,
                                ColliderMask::Custom("BORDER_CUSTOM".to_string()),
                                ColliderMask::Custom("BORDER_TOP".to_string()),
                                ColliderMask::Custom("BORDER_BOTTOM".to_string()),
                                ColliderMask::Custom("BORDER_LEFT".to_string()),
                                ColliderMask::Custom("BORDER_RIGHT".to_string()),
                            ],
                            ColliderType::Rectangle(16, 16),
                        ),
                    ));
                    world.push((
                        Transform::from_xyz(x as f32 * 16. + 26., y as f32 * 16. + 10., 20),
                        Rectangle::new(16., 16., None),
                        Material::Color(Color::new(0, 0, 255, 0.3)),
                        Line { direction: LineDirection::RIGHT },
                        Collider::new(
                            ColliderMask::Landscape,
                            vec![
                                ColliderMask::Bullet,
                                ColliderMask::Custom("BORDER_CUSTOM".to_string()),
                                ColliderMask::Custom("BORDER_TOP".to_string()),
                                ColliderMask::Custom("BORDER_BOTTOM".to_string()),
                                ColliderMask::Custom("BORDER_LEFT".to_string()),
                                ColliderMask::Custom("BORDER_RIGHT".to_string()),
                            ],
                            ColliderType::Rectangle(16, 16),
                        ),
                    ));
                }
            }
        });
    }
}

impl InputLayer {
    fn compute_mouse_on_border(
        &mut self,
        mouse_pos: (f64, f64),
        resources: &mut Resources,
    ) -> bool {
        let (x, y) = mouse_pos;
        if x < 11. {
            if self.state_before_border.is_none() {
                self.state_before_border = Some(self.mouse_state.clone())
            };
            resources.window().set_cursor(CursorIcon::EResize);
            self.mouse_state = CursorState::LEFT;
            true
        } else if x > 1097. {
            if self.state_before_border.is_none() {
                self.state_before_border = Some(self.mouse_state.clone())
            };
            resources.window().set_cursor(CursorIcon::WResize);
            self.mouse_state = CursorState::RIGHT;
            true
        } else if y < 11. {
            if self.state_before_border.is_none() {
                self.state_before_border = Some(self.mouse_state.clone())
            };
            resources.window().set_cursor(CursorIcon::SResize);
            self.mouse_state = CursorState::TOP;
            true
        } else if y > 618. {
            if self.state_before_border.is_none() {
                self.state_before_border = Some(self.mouse_state.clone())
            };
            resources.window().set_cursor(CursorIcon::NResize);
            self.mouse_state = CursorState::BOTTOM;
            true
        } else {
            false
        }
    }

    fn compute_new_icon(&mut self, _resources: &mut Resources) -> CursorIcon {
        if self.mouse_state == CursorState::ROW {
            self.mouse_state = CursorState::COLUMN;
            return CursorIcon::ColResize;
        }
        self.mouse_state = CursorState::ROW;
        CursorIcon::RowResize
    }

    fn change_cursor(&self, resource: &mut Resources) {
        let mut window = resource.window();
        window.set_cursor(match self.mouse_state {
            CursorState::TOP => CursorIcon::NResize,
            CursorState::BOTTOM => CursorIcon::NResize,
            CursorState::LEFT => CursorIcon::WResize,
            CursorState::RIGHT => CursorIcon::WResize,
            CursorState::ROW => CursorIcon::RowResize,
            CursorState::COLUMN => CursorIcon::ColResize,
        })
    }
}
