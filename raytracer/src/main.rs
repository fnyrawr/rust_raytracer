use crate::rt_classes::color::Color;
use crate::rt_classes::samplers::{GradientColor, Disc, PolkaDots, Sampler};
pub mod rt_classes;

/// Push pixel to data vector (append pixel's flattened r, g, b values)
/// #### Arguments
/// * `data`: reference to Vec<f64> reference data array
/// * `color`: pixel color to be appended
fn push_pixel(data: &mut Vec<f64>, color: Color) {
    data.push(color.r);
    data.push(color.g);
    data.push(color.b);
}

/// Convert f64 vector to u8 vector
/// #### Arguments
/// * `input`: Vec<f64> data array reference to be converted
fn f64_to_u8(input: &Vec<f64>) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());

    for src in input {
        let scaled_value = src.round().clamp(0.0, 255.0) as u8;
        output.push(scaled_value);
    }

    output
}

/// Generate and save PNG-Image from Vec<f64> data array
/// #### Arguments
/// * `data`: Vec<f64> data array reference
/// * `width`, `height`: dimensions of the image
/// * `path_str`: relative path as String reference`
fn save_image_png(data: &Vec<f64>, width: usize, height: usize, path_str: &str) {
    // convert float data to int
    let scaled_data = f64_to_u8(data);

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
    writer.write_image_data(&scaled_data).unwrap();
}

#[allow(dead_code)]
fn create_gradient_image(width: usize, height: usize) -> Vec::<f64> {
    let data_size: usize = 3*width*height;

    // data array containing image's flattened pixel data (like [R G B R G B R G B ...])
    let mut data = Vec::<f64>::with_capacity(data_size);

    // assign color to data array (red to green gradient)
    let color1 = Color::new(255.0, 0.0, 0.0);
    let color2 = Color::new(0.0, 255.0, 0.0);
    let gradient_color = GradientColor::new(width, height, color1, color2);
    for y in 0..height {
        for x in 0..width {
            //let color = get_gradient_color(x, y, width, height, color1, color2);
            let color = gradient_color.get_color(x as f64, y as f64);
            push_pixel(&mut data, color);
        }
    }

    data
}

#[allow(dead_code)]
fn create_disc_image(width: usize, height: usize) -> Vec::<f64> {
    let data_size: usize = 3*width*height;

    // data array containing image's flattened pixel data (like [R G B R G B R G B ...])
    let mut data = Vec::<f64>::with_capacity(data_size);

    // assign color to data array (red to green gradient)
    let color = Color::new(255.0, 0.0, 0.0);
    let disc = Disc::new(width, height, height as f64/2.0, color);
    for y in 0..height {
        for x in 0..width {
            //let color = get_gradient_color(x, y, width, height, color1, color2);
            let color = disc.get_color(x as f64, y as f64);
            push_pixel(&mut data, color);
        }
    }

    data
}

#[allow(dead_code)]
fn create_polka_dots_image(width: usize, height: usize) -> Vec::<f64> {
    let data_size: usize = 3*width*height;

    // data array containing image's flattened pixel data (like [R G B R G B R G B ...])
    let mut data = Vec::<f64>::with_capacity(data_size);

    // assign color to data array (red to green gradient)
    let color1 = Color::new(255.0, 0.0, 0.0);
    let color2 = Color::new(255.0, 255.0, 0.0);
    let polka_dots = PolkaDots::new(width, height,16,8,height as f64/8.0, color1, color2);
    for y in 0..height {
        for x in 0..width {
            //let color = get_gradient_color(x, y, width, height, color1, color2);
            let color = polka_dots.get_color(x as f64, y as f64);
            push_pixel(&mut data, color);
        }
    }

    data
}

fn main() {
    // image dimensions
    let width: usize = 1024;
    let height: usize = 576;

    // create image contents
    let data1 = create_gradient_image(width, height);
    let data2 = create_disc_image(width, height);
    let data3 = create_polka_dots_image(width, height);

    // save images
    save_image_png(&data1, width, height, "./doc/gradient_test_image.png");
    save_image_png(&data2, width, height, "./doc/disc_test_image.png");
    save_image_png(&data3, width, height, "./doc/polka_dots_test_image.png");
}
