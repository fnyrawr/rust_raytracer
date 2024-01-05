use crate::rt_classes::color::Color;

/// Generic Sampler trait
pub trait Sampler: Sync + Send {
    /// Return a Color depending on x and y position
    /// #### Arguments
    /// * `x`, `y`: position on x and y axis
    fn get_color(&self, x: f64, y: f64) -> Color;
    fn get_recursion_depth() -> u8;
}

pub struct GradientColor {
    width: usize,
    height: usize,
    start_color: Color,
    end_color: Color,
}

impl GradientColor {
    pub fn new(width: usize, height: usize, start_color: Color, end_color: Color) -> GradientColor {
        GradientColor { width, height, start_color, end_color }
    }
}

impl Sampler for GradientColor {
    fn get_color(&self, x: f64, y: f64) -> Color {
        let w = (x + y) / (self.width + self.height) as f64;
        Color::add(&self.start_color, &Color::multiply(w, &Color::subtract(&self.end_color, &self.start_color)))
    }

    fn get_recursion_depth() -> u8 {
        return 0
    }
}

#[derive(Clone)]
pub struct Disc {
    center_x: f64,
    center_y: f64,
    size: f64,
    color: Color,
}

impl Disc {
    pub fn new(width: usize, height: usize, size: f64, color: Color) -> Disc {
        Disc { center_x: width as f64/2.0, center_y: height as f64/2.0, size, color }
    }
}

impl Sampler for Disc {
    fn get_color(&self, x: f64, y: f64) -> Color {
        // if ((x-center_x)^2 + (y-center_y)^2) <= radius^2, point is on disc
        if (x-self.center_x)*(x-self.center_x) + (y-self.center_y)*(y-self.center_y) <= self.size/2.0 * self.size/2.0 {
            return self.color;
        }
        // return black
        Color::new(0.0, 0.0, 0.0)
    }

    fn get_recursion_depth() -> u8 {
        return 0
    }
}

#[derive(Clone)]
pub struct PolkaDots {
    width: usize,
    height: usize,
    size: f64,
    start_color: Color,
    end_color: Color,
    step_x: f64,
    step_y: f64,
    center_x: f64,
    center_y: f64,
}

impl PolkaDots {
    pub fn new(width: usize, height: usize, count_x: u8, count_y: u8, size: f64, start_color: Color, end_color: Color) -> PolkaDots {
        PolkaDots {
            width, height, size, start_color, end_color,
            step_x: width as f64 / count_x as f64,
            step_y: height as f64 / count_y as f64,
            center_x: width as f64 / count_x as f64 / 2.0,
            center_y: height as f64 / count_y as f64 / 2.0
        }
    }
}

impl Sampler for PolkaDots {
    fn get_color(&self, x: f64, y: f64) -> Color {
        // get relative position on pattern
        let px = x % self.step_x;
        let py = y % self.step_y;

        // color = start_color + w*(end_color-start_color) | w = (x+y/width+height)
        let w = (x + y) / (self.width + self.height) as f64;
        let color = Color::add(&self.start_color, &Color::multiply(w, &Color::subtract(&self.end_color, &self.start_color)));

        // if ((px-center_x)^2 + (py-center_y)^2) <= radius^2, point is on dot
        if (px-self.center_x)*(px-self.center_x) + (py-self.center_y)*(py-self.center_y) <= self.size/3.0 * self.size/2.0 {
            return color;
        }
        // return black
        Color::new(0.0, 0.0, 0.0)
    }

    fn get_recursion_depth() -> u8 {
        return 0
    }
}