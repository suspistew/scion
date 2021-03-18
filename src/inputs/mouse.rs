#[derive(Default, Debug)]
pub struct Mouse {
    x: f64,
    y: f64,
}

impl Mouse {
    pub fn set_position(&mut self, x: f64, y: f64){
        self.x = x;
        self.y = y;
    }
}