use crate::rt_classes::color::Color;

pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
    gamma: f64,
}

#[allow(dead_code)]
impl Image {
    pub fn new(width: usize, height: usize, gamma: f64) -> Image {
        let size = 3*width*height;
        let mut vec: Vec<u8> = Vec::<u8>::with_capacity(size);
        for _i in 0..size { vec.push(0); }
        Image { data: vec, width, height, gamma }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = 3 * (y * self.width + x);

        if self.gamma != 0.0 {
            let gm = 1.0 / self.gamma;
            self.data[index] = (f64::powf(color.r, gm) * 255f64).round() as u8;
            self.data[index + 1] = (f64::powf(color.g, gm) * 255f64).round() as u8;
            self.data[index + 2] = (f64::powf(color.b, gm) * 255f64).round() as u8;
        }
        else {
            self.data[index] = (color.r * 255f64).round() as u8;
            self.data[index + 1] = (color.g * 255f64).round() as u8;
            self.data[index + 2] = (color.b * 255f64).round() as u8;
        }
    }

    pub fn get_data(&self) -> Vec<u8> {
        let output = self.data.clone();
        output
    }
}