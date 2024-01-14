use crate::rt_classes::matrix::Point;
use crate::rt_classes::ray::Ray;

pub trait Shape {
    fn intersect(r: Ray) -> Point;
    // fn bounds(&self) -> BoundingBox;
    // fn calculate_bounds(&self) -> BoundingBox;
}