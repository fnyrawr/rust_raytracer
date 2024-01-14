use crate::rt_classes::color::Color;
use crate::rt_classes::hit::Hit;
use crate::rt_classes::ray::Ray;
use crate::rt_classes::samplers::{ConstantColor, Sampler};

pub trait Material {
    fn get_emmission(&self, hit: Hit) -> Color;
    fn get_albedo(&self, hit: Hit) -> Color;
    fn get_secondary_ray(&self, ray: Ray, hit: Hit) -> Ray;
}

pub struct EmmittingMaterial {
    emmission: dyn Sampler,
}

impl EmmittingMaterial {
    pub fn new(color: Color) -> EmmittingMaterial {
        EmmittingMaterial { emmission: ConstantColor::new(color) }
    }
}

impl Material for EmmittingMaterial {
    fn get_emmission(&self, hit: Hit) -> Color {
        // color of sky in ray direction, constant for now
        self.emmission.get_color(hit.u, hit.v)
    }

    fn get_albedo(&self, hit: Hit) -> Color {
        // no diffusion, no albedo
        None
    }

    fn get_secondary_ray(&self, ray: Ray, hit: Hit) -> Ray {
        // no secondary ray as there is no diffusion going on
        None
    }
}