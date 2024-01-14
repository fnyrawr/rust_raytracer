use crate::rt_classes::materials::Material;
use crate::rt_classes::matrix::{Direction, Point};

pub struct Hit {
    pub t: f64,
    pub p: Point,
    pub n: Direction,
    pub m: Box<dyn Material>,
    pub u: f64,
    pub v: f64,
}

#[allow(dead_code)]
impl Hit {
    pub fn new(t: f64, x: Point, n: Direction, m: Box<dyn Material>) -> Hit {
        Hit { t, p: x, n, u: 0.0, v: 0.0, m }
    }

    pub fn new_with_uv(t: f64, x: Point, n: Direction, u: f64, v: f64, m: Box<dyn Material>) -> Hit {
        Hit { t, p: x, n, u, v, m }
    }

    pub fn get_distance(&self) -> f64 {
        self.t
    }

    pub fn get_hitpoint(&self) -> Point {
        self.p
    }

    pub fn get_normal(&self) -> Direction {
        self.n
    }

    pub fn get_u(&self) -> f64 {
        self.u
    }

    pub fn get_v(&self) -> f64 {
        self.v
    }
}