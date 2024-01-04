/// Representation of a simple RGB Color
#[derive(Debug, Clone, Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

#[allow(dead_code)]
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color{ r, g, b }
    }

    /// Add two colors returning a new color instance
    /// ### Arguments
    /// * `a`, `b`: Color references
    fn add(a: &Color, b: &Color) -> Color {
        Color {
            r: a.r + b.r,
            g: a.g + b.g,
            b: a.b + b.b,
        }
    }

    /// Subtract color b from color a returning a new color instance
    /// ### Arguments
    /// * `a`, `b`: Color references
    fn subtract(a: &Color, b: &Color) -> Color {
        Color {
            r: a.r - b.r,
            g: a.g - b.g,
            b: a.b - b.b,
        }
    }

    /// multiply a color by a scalar returning a new color instance
    /// ### Arguments
    /// * `s`: scalar
    /// * `a`: Color reference
    fn multiply(s: f64, a: &Color) -> Color {
        Color {
            r: s * a.r,
            g: s * a.g,
            b: s * a.b,
        }
    }

    /// divide a color by a scalar returning a new color instance
    /// ### Arguments
    /// * `a`: Color reference
    /// * `s`: scalar
    fn divide(a: &Color, s: f64) -> Color {
        Color {
            r: a.r / s,
            g: a.g / s,
            b: a.b / s,
        }
    }

    /// clamp color values between 0 and 1 returning a new color instance
    /// ### Arguments
    /// * `v`: Color reference
    fn clamp(v: &Color) -> Color {
        Color {
            r: v.r.min(1.0).max(0.0),
            g: v.g.min(1.0).max(0.0),
            b: v.b.min(1.0).max(0.0),
        }
    }
}

/// Sampler method to get a Color between two given colors depending on position (diagonal gradient between 0,0 and max_x, max_y)
/// ### Arguments
/// * `x`, `y`: position on x and y axis
/// * `width`, `height`: image dimensions
/// * `color1`, `color2`: start and end Color of gradient
fn get_gradient_color(x: usize, y: usize, width: usize, height: usize, color1: Color, color2: Color) -> Color {
    // assign diagonal gradient
    let w = (x as f64 + y as f64) / (width + height) as f64;
    Color::add(&color1, &Color::multiply(w, &Color::subtract(&color2, &color1)))
}

/// Push pixel to data vector (append pixel's flattened r, g, b values)
/// ### Arguments
/// * `data`: reference to Vec<f64> reference data array
/// * `color`: pixel color to be appended
fn push_pixel(data: &mut Vec<f64>, color: Color) {
    data.push(color.r);
    data.push(color.g);
    data.push(color.b);
}

/// Convert f64 vector to u8 vector
/// ### Arguments
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
/// ### Arguments
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

fn main() {
    // image dimensions
    let width: usize = 1920;
    let height: usize = 1080;
    let data_size: usize = 3*width*height;

    // data array containing image's flattened pixel data (like [R G B R G B R G B ...])
    //let mut data: [f64; DATA_SIZE] = [0.0; DATA_SIZE];
    let mut data = Vec::<f64>::with_capacity(data_size);

    // assign color to data array (red to green gradient)
    let color1 = Color::new(255.0, 0.0, 0.0);
    let color2 = Color::new(0.0, 255.0, 0.0);
    for y in 0..height {
        for x in 0..width {
            let color = get_gradient_color(x, y, width, height, color1, color2);
            push_pixel(&mut data, color);
        }
    }

    // save image
    save_image_png(&data, width, height, "./doc/test_image.png");
}
