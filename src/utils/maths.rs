
use serde::{Deserialize, Serialize};
use crate::core::components::maths::coordinates::Coordinates;

/// The standard way to communicate a position in 3 dimensions in `Scion`
#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl Position {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn z(&self) -> usize {
        self.z
    }
}

/// The standard way to communicate 3D sizes in `Scion`
pub struct Dimensions {
    width: usize,
    height: usize,
    depth: usize,
}

impl Dimensions {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self { width, height, depth }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

/// Struct used in all `Scion` to specify any 2D movement.
#[derive(Default, Debug, Copy, Clone)]
pub struct Vector {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}


// https://www.omnicalculator.com/math/centroid#what-is-the-formula-for-the-centroid
pub fn centroid_polygon(vertices: &Vec<Coordinates>) -> Coordinates {
    let (mut x, mut y) = (0., 0.);
    let mut signed_area: f32 = 0.;

    for i in 0..vertices.len() {
        let current_vertice = vertices.get(i).unwrap();
        let next_vertice = vertices.get((i + 1) % vertices.len()).unwrap();
        let current_area = (current_vertice.x * next_vertice.y) - (next_vertice.x * current_vertice.y);
        signed_area += current_area;
        x += (current_vertice.x + next_vertice.x) * current_area;
        y += (current_vertice.y + next_vertice.y) * current_area;
    }
    signed_area = signed_area * 0.5;
    x = x / (6. * signed_area);
    y = y / (6. * signed_area);
    Coordinates::new(x, y)
}

// https://www.omnicalculator.com/math/centroid#what-is-the-formula-for-the-centroid
pub fn centroid_points(vertices: &Vec<Coordinates>) -> Coordinates {
    let sum = vertices.iter().fold(Coordinates::default(),
                                   |acc, current|
                                       Coordinates::new(acc.x + current.x, current.y + acc.y));
    Coordinates::new(sum.x / vertices.len() as f32, sum.y / vertices.len() as f32)
}

pub fn rotate_point_around_pivot(point: &Coordinates, pivot: &Coordinates, angle: f32) -> Coordinates{
    if angle != 0.{
        let angle_rad = angle;
        let x2 = pivot.x + (point.x - pivot.x) * angle_rad.cos() - (point.y - pivot.y) * angle_rad.sin();
        let y2 = pivot.y + (point.x - pivot.x) * angle_rad.sin() + (point.y - pivot.y) * angle_rad.cos();
        Coordinates::new(x2, y2)
    }else{
        Coordinates::new(point.x, point.y)
    }

}

#[cfg(test)]
mod test {
    use crate::core::components::maths::coordinates::Coordinates;
    use crate::utils::maths::{centroid_points, centroid_polygon, rotate_point_around_pivot};

    #[test]
    fn test_centroid() {
        let vertices = vec![Coordinates::new(1., 1.), Coordinates::new(2., 4.), Coordinates::new(5., 4.), Coordinates::new(11., 1.)];
        println!("Centre par polygone : {:?}", centroid_polygon(&vertices));
    }

    #[test]
    fn test_centroid2() {
        let vertices = vec![Coordinates::new(1., 1.), Coordinates::new(2., 4.), Coordinates::new(5., 4.), Coordinates::new(11., 1.)];
        println!("Centre par points : {:?}", centroid_points(&vertices));
    }

    #[test]
    fn test_pivot() {
        let angle :f32 = 45.;
        let angle = angle.to_radians();
        let r = rotate_point_around_pivot(&Coordinates::new(128.,681.), &Coordinates::new(96., 681.), angle );
        println!("{:?}", r);
    }
}