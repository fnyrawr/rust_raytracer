use png::ScaledFloat;
use crate::rt_classes::color::Color;

pub struct Image {
    data: Vec<f64>,
    width: usize,
    height: usize,
    gamma: f32,
}

#[allow(dead_code)]
impl Image {
    pub fn new(width: usize, height: usize, gamma: f32) -> Image {
        let size = 3*width*height;
        let mut vec: Vec<f64> = Vec::<f64>::with_capacity(size);
        for _i in 0..size { vec.push(0.0); }
        Image { data: vec, width, height, gamma }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = 3 * (y * self.width + x);

        self.data[index] = color.r;
        self.data[index + 1] = color.g;
        self.data[index + 2] = color.b;
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(3*self.width*self.height);

        for src in &self.data {
            let scaled_value = src.round().clamp(0.0, 255.0) as u8;
            output.push(scaled_value);
        }

        output
    }

    pub fn get_gamma(&self) -> ScaledFloat {
        ScaledFloat::new(self.gamma)
    }
}