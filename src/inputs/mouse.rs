#[derive(Default, Debug)]
pub struct Mouse {
    x: f64,
    y: f64,
    click_event: bool,
}

impl Mouse {
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    pub fn set_click_event(&mut self, click: bool) {
        self.click_event = click;
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn click_event(&self) -> bool {
        self.click_event
    }
}
