use crate::rt_classes::color::Color;
use crate::rt_classes::image::Image;
use crate::rt_classes::samplers::ColoredDiscs;
pub mod rt_classes;

fn main() {
    // image dimensions
    let width: usize = 1920;
    let height: usize = 1080;
    let gamma: f64 = 2.2;

    // create new image
    let mut image = Image::new(width, height, gamma);

    // sample the image
    let color1 = Color::new_from_u8(15, 88, 186);
    let color2 = Color::new_from_u8(173, 216, 230);
    let colored_discs = ColoredDiscs::new(width, height, 30, color1, color2);
    image.sample(&colored_discs, 32);

    // save image output to PNG
    image.save_as_png("./doc/colored_discs_test_image.png");
}
