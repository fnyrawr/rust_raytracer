use crate::rt_classes::vector::{Direction, Point, Vector};

pub struct Ray {
    // origin: x0 | direction: d | ray path: t_min -> t_max
    origin: Point,
    direction: Direction,
    t_min: f64,
    t_max: f64,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Point, direction: Direction, t_min: f64, t_max: f64) -> Ray {
        Ray { origin, direction, t_min, t_max }
    }

    pub fn point_at(&self, t: f64) -> Point {
        // x(t) = x0 + t*d
        Vector::add_point_direction(self.origin, Vector::multiply_scalar_direction(t, self.direction))
    }

    pub fn contains(&self, t: f64) -> bool {
        if (t >= self.t_min) && (t <= self.t_max) {
            return true
        }
        false
    }

    pub fn get_origin(&self) -> Point {
        self.origin
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }
}