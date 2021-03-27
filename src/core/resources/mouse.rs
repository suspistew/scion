/// Contains some data about the mouse, updated at each event received from the window.
/// Can be used in any system.
#[derive(Default, Debug)]
pub struct Mouse {
    x: f64,
    y: f64,
    click_event: bool,
}

impl Mouse {
    pub(crate) fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    pub(crate) fn set_click_event(&mut self, click: bool) {
        self.click_event = click;
    }

    /// Returns the current x value of the cursor
    pub fn x(&self) -> f64 {
        self.x
    }
    /// Returns the current y value of the cursor
    pub fn y(&self) -> f64 {
        self.y
    }
    /// Returns if the mouse has been clicked in the current frame
    pub fn click_event(&self) -> bool {
        self.click_event
    }
}
