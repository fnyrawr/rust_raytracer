use crate::rt_classes::color::Color;
use crate::rt_classes::image::Image;
use crate::rt_classes::samplers::{GradientColor};
pub mod rt_classes;

fn main() {
    // image dimensions
    let width: usize = 1024;
    let height: usize = 576;
    let gamma: f64 = 2.2;

    // create new image
    let mut image = Image::new(width, height, gamma);

    // sample the image
    let color1 = Color::new_from_u8(255, 0, 0);
    let color2 = Color::new_from_u8(0, 255, 255);
    let gradient = GradientColor::new(width, height, color1, color2);
    image.sample(&gradient, 0);

    // save image output to PNG
    image.save_as_png("./doc/tmp.png");
}
