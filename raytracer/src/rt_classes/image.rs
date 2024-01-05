use crate::rt_classes::color::Color;
use crate::rt_classes::samplers::Sampler;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::time:: Instant;

pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
    gamma: f64,
    rng: XorShiftRng,
}

#[allow(dead_code)]
impl Image {
    pub fn new(width: usize, height: usize, gamma: f64) -> Image {
        let size = 3*width*height;
        let mut vec: Vec<u8> = Vec::<u8>::with_capacity(size);
        for _i in 0..size { vec.push(0); }
        Image { data: vec, width, height, gamma, rng: XorShiftRng::from_entropy() }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
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

    pub fn sample(&mut self, sampler: &dyn Sampler, n: u8) {
        println!("Creating image | Dimensions {} x {} | {}x Antialiasing", self.width, self.height, n);
        let start = Instant::now();

        // precompute random numbers reducing overhead
        let mut random_values_x = Vec::with_capacity(n as usize);
        let mut random_values_y = Vec::with_capacity(n as usize);
        for _ in 0..n {
            random_values_x.push(self.rng.gen_range(0.0..1.0));
            random_values_y.push(self.rng.gen_range(0.0..1.0));
        }

        let mut i = 0;
        let k = self.height * self.width;
        for y in 0..self.height {
            for x in 0..self.width {
                let color = if n > 0 {
                    // Antialiasing with n subpixels
                    let mut color = Color::new(0.0, 0.0, 0.0);
                    for i in 0..n {
                        for j in 0..n {
                            let xs = x as f64 + random_values_x[i as usize];
                            let ys = y as f64 + random_values_y[j as usize];
                            color = Color::add(&color, &sampler.get_color(xs, ys));
                        }
                    }
                    let n = n as f64;
                    Color::divide(&color, n * n)
                }
                else {
                    sampler.get_color(x as f64, y as f64)
                };
                self.set_pixel(x, y, color);
                i += 1;
                if i % 1000 == 0 {
                    let p: f32 = (i * 100) as f32 / k as f32;
                    print!("\rSampling ...   | {:.2}% done | {:3} seconds elapsed", p, start.elapsed().as_secs());
                }

            }
        }
        let duration = start.elapsed();
        println!("\rImage creation finished in {:?} seconds                              ", duration);
    }

    pub fn save_as_png(&self, path_str: &str) {
        use std::io::BufWriter;
        use std::path::Path;
        use std::fs::File;

        let path = Path::new(&path_str);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
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
        writer.write_image_data(&*self.data).unwrap();
    }
}