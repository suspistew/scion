use winit::window::CursorIcon;

/// [`Window`] is a Resource dedicated to have an access control over the current window.
/// Its size is immediatly updated when the window resize event happens.
/// new_cursor is set at the end of the current frame.
#[derive(Default, Debug, Copy, Clone)]
pub struct Window {
    width: u32,
    height: u32,
    dpi: f64,
    new_cursor: Option<CursorIcon>,
}

impl Window {
    pub(crate) fn new(screen_size: (u32, u32), dpi: f64) -> Self {
        Self { width: screen_size.0, height: screen_size.1, new_cursor: None, dpi }
    }

    pub(crate) fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_cursor(&mut self, icon: CursorIcon) { self.new_cursor = Some(icon); }

    pub(crate) fn reset_new_cursor(&mut self) { self.new_cursor = None }

    pub fn dimensions(&self) -> (u32, u32) { (self.width, self.height) }

    pub fn width(&self) -> u32 { self.width }

    pub fn height(&self) -> u32 { self.height }

    pub fn dpi(&self) -> f64 { self.dpi }

    pub fn new_cursor(&self) -> &Option<CursorIcon> { &self.new_cursor }
}
