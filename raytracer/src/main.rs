#[allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color{ r, g, b }
    }

    fn add(a: Color, b: Color) -> Color {
        let mut result = Color {
            r: a.r + b.r,
            g: a.g + b.g,
            b: a.b + b.b,
        };

        result
    }

    fn subtract(a: Color, b: Color) -> Color {
        let mut result = Color {
            r: a.r - b.r,
            g: a.g - b.g,
            b: a.b - b.b,
        };

        result
    }

    fn multiply(s: f64, a: Color) -> Color {
        Color {
            r: s * a.r,
            g: s * a.g,
            b: s * a.b,
        }
    }

    fn divide(a: Color, s: f64) -> Color {
        Color {
            r: a.r / s,
            g: a.g / s,
            b: a.b / s,
        }
    }

    fn clamp(v: Color) -> Color {
        Color {
            r: v.r.min(1.0).max(0.0),
            g: v.g.min(1.0).max(0.0),
            b: v.b.min(1.0).max(0.0),
        }
    }
}

fn get_gradient_color(x: usize, y: usize, width: usize, height: usize, color1: Color, color2: Color) -> Color {
    // assign diagonal gradient
    let w = (x as f64 + y as f64) / (width + height) as f64;
    let color = Color::add(color1, Color::multiply(w, Color::subtract(color2, color1)));

    color
}

fn set_pixel(data: &mut [f64], x: usize, y: usize, width: usize, height: usize, color: Color) {
    let i = 3 * (y * width + x);
    data[i] = color.r;
    data[i+1] = color.g;
    data[i+2] = color.b;
}

fn f64_to_u8(input: &[f64]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());

    for src in input {
        let scaled_value = src.round().clamp(0.0, 255.0) as u8;
        output.push(scaled_value);
    }

    output
}

fn save_image_png(data: &[f64], width: usize, height: usize, path_str: &str) {
    // convert float data to int
    let scaled_data = f64_to_u8(&data);

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
    const WIDTH: usize = 400;
    const HEIGHT: usize = 200;
    const DATA_SIZE: usize = 3*WIDTH*HEIGHT;

    // data array containing image's flattened pixel data (like [R G B R G B R G B ...])
    let mut data: [f64; DATA_SIZE] = [0.0; DATA_SIZE];

    // assign color to data array (red to green gradient)
    let color1 = Color::new(255.0, 0.0, 0.0);
    let color2 = Color::new(0.0, 255.0, 0.0);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = get_gradient_color(x, y, WIDTH, HEIGHT, color1, color2);
            set_pixel(&mut data, x, y, WIDTH, HEIGHT, color);
        }
    }

    // save image
    save_image_png(&data, WIDTH, HEIGHT, "./doc/test_image.png");
}
