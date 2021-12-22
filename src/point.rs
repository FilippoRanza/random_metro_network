use super::rand_utils::random_in;
use super::Localizable;

pub type Coord = (f64, f64);

pub fn coord_to_point(c: Coord) -> Point {
    Point { x: c.0, y: c.1 }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn random_point(sz: f64) -> Self {
        let x = random_in(2. * sz);
        let y = random_in(2. * sz);

        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let (x1, y1) = self.coordinates();
        let (x2, y2) = other.coordinates();
        let dx = x1 - x2;
        let dy = y1 - y2;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn norm(&self) -> f64 {
        let (x, y) = self.coordinates();
        (x * x + y * y).sqrt()
    }

    pub fn translate(&self, dist: f64, angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        let dx = c * dist;
        let dy = s * dist;
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn is_inside(&self, map_size: f64) -> bool {
        self.x.abs() < map_size && self.y.abs() < map_size
    }

    pub fn find_middle(&self, other: &Self) -> Self {
        let (x1, y1) = self.coordinates();
        let (x2, y2) = other.coordinates();
        let x = (x1 + x2) / 2.;
        let y = (y1 + y2) / 2.;
        Self { x, y }
    }
}

impl Localizable for Point {
    fn coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }
}
