use winit::event::WindowEvent;
use crate::core::scion_runner::ScionRunner;
use crate::graphics::windowing::input_event_handler::update_input_events;

pub fn handle_window_event(runner: &mut ScionRunner) -> bool {
    let mut force_redraw = false;
    if let Some(receiver) = runner.main_thread_receiver.as_ref() {
        while let Ok(event) = receiver.try_recv() {
            if event.redraw {
                force_redraw = true;
            };
            if let Some(window_event) = event.window_event {
                match window_event {
                    WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                        runner.renderer.as_mut().unwrap().resize(
                            runner.window.as_ref().expect("Missing window").inner_size(),
                            scale_factor,
                        );
                        force_redraw = true;
                    }
                    WindowEvent::Resized(physical_size) => {
                        runner.game_data
                            .window()
                            .set_dimensions(physical_size.width, physical_size.height);
                        runner.game_data
                            .window()
                            .set_dpi(runner.window.as_ref().expect("Missing window").scale_factor());
                        runner.renderer.as_mut().unwrap().resize(
                            physical_size,
                            runner.window.as_ref().expect("Missing window").scale_factor(),
                        );
                        force_redraw = true;
                    }
                    e => update_input_events(e, &mut runner.game_data)
                }
            }
        }
    }
    force_redraw
}