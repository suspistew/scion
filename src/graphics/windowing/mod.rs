use winit::event::WindowEvent;

pub(crate) mod window_event_handler;

#[derive(Debug)]
pub struct WindowingEvent {
    pub(crate) window_event: Option<WindowEvent>,
    pub(crate) redraw: bool
}
