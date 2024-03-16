#[derive(Default, Copy, Clone)]
pub struct Padding {
    pub(crate) top: Option<f32>,
    pub(crate) left: Option<f32>,
}

impl Padding {
    pub fn new(top: Option<f32>, left: Option<f32>) -> Self {
        Self{
            top,
            left,
        }
    }

    pub fn top_or_zero(&self) -> f32 {
        self.top.unwrap_or(0.)
    }

    pub fn left_or_zero(&self) -> f32 {
        self.left.unwrap_or(0.)
    }
}