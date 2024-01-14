use crate::rt_classes::color::Color;
use crate::rt_classes::samplers::{ConstantColor, Sampler};
use crate::rt_classes::materials::Material;

pub struct Background {
    background: Box<dyn Sampler>,
    material: Box<dyn Material>,
    // bounding_box: BoundingBox
}

impl Background {
    pub fn new(&self, color: Color) -> Box<Background> {
        Box::new(Background {
            background: Box::new(ConstantColor::new(color)),
            material: Box::new(self.material),
        })
    }
}