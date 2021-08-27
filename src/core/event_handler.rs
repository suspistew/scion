use legion::{Resources, World};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::{
    config::scion_config::ScionConfig,
    core::{
        legion_ext::ScionResourcesExtension,
        resources::{
            inputs::{
                keycode::KeyCode,
                mouse::{MouseButton, MouseEvent},
                InputState, KeyboardEvent,
            },
        },
    },
    rendering::renderer_state::RendererState,
};

pub(crate) fn handle_event(
    event: Event<()>,
    control_flow: &mut ControlFlow,
    window: &mut Window,
    renderer: &mut RendererState,
    world: &mut World,
    resources: &mut Resources,
    config: &ScionConfig,
) {
    match event {
        Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    resources.window().set_dimensions(physical_size.width, physical_size.height);
                    renderer.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    renderer.resize(**new_inner_size);
                }
                WindowEvent::CursorMoved { device_id: _, position, .. } => {
                    let dpi_factor =
                        window.current_monitor().expect("Missing the monitor").scale_factor();
                    resources
                        .inputs()
                        .mouse_mut()
                        .set_position(position.x / dpi_factor, position.y / dpi_factor);
                }
                _ => {}
            }
            update_input_events(event, resources);
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            renderer.update(world, resources);
            match renderer.render(world, config) {
                Ok(_) => {}
                Err(e) => log::error!("{:?}", e),
            }
        }
        _ => (),
    }
}

fn update_input_events(window_event: &WindowEvent, resources: &mut Resources) {
    match window_event {
        WindowEvent::KeyboardInput { input, .. } => {
            if let Some(keycode) = input.virtual_keycode {
                let k_event = KeyboardEvent {
                    keycode: KeyCode::from(keycode),
                    state: InputState::from(input.state),
                };
                resources.inputs().keyboard_mut().add_keyboard_event(k_event.clone());
            }
        }
        WindowEvent::MouseInput { state, button, .. } => {
            let m_event =
                MouseEvent { button: MouseButton::from(*button), state: InputState::from(*state) };
            resources.inputs().mouse_mut().set_click_event(Some(m_event.clone()));
        }
        _ => {}
    };
}
