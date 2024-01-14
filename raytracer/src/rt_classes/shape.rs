use crate::rt_classes::hit::Hit;
use crate::rt_classes::ray::Ray;

pub trait Shape {
    fn intersect(&self, ray: Ray) -> Hit;
    // fn bounds(&self) -> BoundingBox;
    // fn calculate_bounds(&self) -> BoundingBox;
}