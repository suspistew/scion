pub(crate) mod window_event_handler;
pub(crate) mod input_event_handler;

use winit::event::WindowEvent;
use winit::window::WindowId;

#[derive(Debug)]
pub struct WindowingEvent {
    pub(crate) window_event: Option<WindowEvent>,
    pub(crate) id: Option<WindowId>,
    pub(crate) redraw: bool
}
