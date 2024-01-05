use crate::rt_classes::color::Color;
use crate::rt_classes::image::Image;
use crate::rt_classes::samplers::{PolkaDots};
pub mod rt_classes;

fn main() {
    // image dimensions
    let width: usize = 1920;
    let height: usize = 1080;
    let gamma: f64 = 2.2;

    // create new image
    let mut image = Image::new(width, height, gamma);

    // sample the image
    let color1 = Color::new_from_u8(255, 255, 0);
    let color2 = Color::new_from_u8(0, 0, 255);
    let polkadots = PolkaDots::new(width, height, 16, 8, height as f64 / 8f64, color1, color2);
    image.sample(&polkadots, 64);

    // save image output to PNG
    image.save_as_png("./doc/tmp.png");
}
