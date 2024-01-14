use crate::rt_classes::color::Color;
use crate::rt_classes::hit::Hit;
use crate::rt_classes::samplers::{ConstantColor, Sampler};
use crate::rt_classes::materials::{EmmittingMaterial, Material};
use crate::rt_classes::ray::Ray;
use crate::rt_classes::shape::Shape;
use crate::rt_classes::vector::Vector;

pub struct Background {
    background: Box<dyn Sampler>,
    material: Box<dyn Material>,
    // bounding_box: BoundingBox
}

#[allow(dead_code)]
impl Background {
    pub fn new(color: Color) -> Background {
        Background {
            background: Box::new(ConstantColor::new(color)),
            material: Box::new(EmmittingMaterial::new(color))
        }
    }
}

#[allow(dead_code)]
impl Shape for Background {
    fn intersect(&self, ray: Ray) -> Hit {
        let x = Vector::point(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let normal = Vector::negate_direction(ray.get_direction());
        let inclination = ray.get_direction().y.acos();
        let azimuth = std::f64::consts::PI + ray.get_direction().x.atan2(ray.get_direction().z);
        let u = azimuth / (2f64*std::f64::consts::PI);
        let v = inclination / std::f64::consts::PI;
        Hit::new_with_uv(f64::INFINITY, x, normal, u, v, Box::new(self.material.clone()))
    }
}