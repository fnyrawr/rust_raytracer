use crate::rt_classes::color::Color;
use crate::rt_classes::image::Image;
use crate::rt_classes::samplers::{GradientColor, Disc, PolkaDots, Sampler};
pub mod rt_classes;

/// Generate and save PNG-Image from Vec<u8> data array
/// #### Arguments
/// * `data`: Vec<u8> data array reference
/// * `width`, `height`: dimensions of the image
/// * `path_str`: relative path as String reference`
fn save_image_png(data: &Vec<u8>, width: usize, height: usize, path_str: &str) {
    use std::io::BufWriter;
    use std::path::Path;
    use std::fs::File;

    let path = Path::new(&path_str);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let source_chromaticities = png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
}

#[allow(dead_code)]
fn create_gradient_image(width: usize, height: usize, gamma: f64) {
    // create new image
    let mut image = Image::new(width, height, gamma);

    // sample the image
    let color1 = Color::new_from_u8(255, 0, 0);
    let color2 = Color::new_from_u8(0, 255, 0);
    let gradient_color = GradientColor::new(width, height, color1, color2);
    for y in 0..height {
        for x in 0..width {
            let color = gradient_color.get_color(x as f64, y as f64);
            image.set_pixel(x, y, color);
        }
    }

    // save image output to PNG
    save_image_png(&image.get_data(), width, height, "./doc/gradient_test_image.png");
}

#[allow(dead_code)]
fn create_disc_image(width: usize, height: usize, gamma: f64) {
    // create new image
    let mut image = Image::new(width, height, gamma);

    // sample the image
    let color = Color::new_from_u8(255, 0, 0);
    let disc = Disc::new(width, height, height as f64/2.0, color);
    for y in 0..height {
        for x in 0..width {
            let color = disc.get_color(x as f64, y as f64);
            image.set_pixel(x, y, color);
        }
    }

    // save image output to PNG
    save_image_png(&image.get_data(), width, height, "./doc/disc_test_image.png");
}

#[allow(dead_code)]
fn create_polka_dots_image(width: usize, height: usize, gamma: f64) {
    // create new image
    let mut image = Image::new(width, height, gamma);

    // sample the image
    let color1 = Color::new_from_u8(255, 0, 0);
    let color2 = Color::new_from_u8(255, 255, 0);
    let polka_dots = PolkaDots::new(width, height,16,8,height as f64/8.0, color1, color2);
    for y in 0..height {
        for x in 0..width {
            //let color = get_gradient_color(x, y, width, height, color1, color2);
            let color = polka_dots.get_color(x as f64, y as f64);
            image.set_pixel(x, y, color);
        }
    }

    // save image output to PNG
    save_image_png(&image.get_data(), width, height, "./doc/polka_dots_test_image.png");
}

fn main() {
    // image dimensions
    let width: usize = 1024;
    let height: usize = 576;
    let gamma: f64 = 2.2;

    // create the images
    create_gradient_image(width, height, gamma);
    create_disc_image(width, height, gamma);
    create_polka_dots_image(width, height, gamma);
}
