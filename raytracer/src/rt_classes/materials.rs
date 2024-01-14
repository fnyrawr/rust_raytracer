use crate::rt_classes::color::Color;
use crate::rt_classes::hit::Hit;
use crate::rt_classes::ray::Ray;
use crate::rt_classes::samplers::{ConstantColor, Sampler};

pub trait Material {
    fn get_emmission(&self, hit: Hit) -> Option<Color>;
    fn get_albedo(&self, hit: Hit) -> Option<Color>;
    fn get_secondary_ray(&self, ray: Ray, hit: Hit) -> Option<Ray>;
}

pub struct EmmittingMaterial {
    emmission: ConstantColor,
}

impl EmmittingMaterial {
    pub fn new(color: Color) -> EmmittingMaterial {
        EmmittingMaterial { emmission: ConstantColor::new(color) }
    }
}

impl Material for EmmittingMaterial {
    fn get_emmission(&self, hit: Hit) -> Option<Color> {
        // color of sky in ray direction, constant for now
        Option::from(self.emmission.get_color(hit.u, hit.v))
    }

    fn get_albedo(&self, _hit: Hit) -> Option<Color> {
        // no diffusion, no albedo
        None
    }

    fn get_secondary_ray(&self, _ray: Ray, _hit: Hit) -> Option<Ray> {
        // no secondary ray as there is no diffusion going on
        None
    }
}