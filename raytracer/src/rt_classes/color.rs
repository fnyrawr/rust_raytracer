/// Representation of a simple RGB Color in scale 0..1
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[allow(dead_code)]
impl Color {
    /// Create a new color instance
    /// #### Arguments
    /// * `r`, `g`, `b`: Red, Green, Blue in range 0..1
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    /// Create a new color instance
    /// #### Arguments
    /// * `r`, `g`, `b`: Red, Green, Blue in range 0..255
    pub fn new_from_u8(r: u8, g: u8, b: u8) -> Color { Color { r: r as f64/255f64, g: g as f64/255f64, b: b as f64/255f64 } }

    /// Add two colors returning a new color instance
    /// #### Arguments
    /// * `a`, `b`: Color references
    pub fn add(a: &Color, b: &Color) -> Color {
        Color {
            r: a.r + b.r,
            g: a.g + b.g,
            b: a.b + b.b,
        }
    }

    /// Subtract color b from color a returning a new color instance
    /// #### Arguments
    /// * `a`, `b`: Color references
    pub fn subtract(a: &Color, b: &Color) -> Color {
        Color {
            r: a.r - b.r,
            g: a.g - b.g,
            b: a.b - b.b,
        }
    }

    /// multiply a color by a scalar returning a new color instance
    /// #### Arguments
    /// * `s`: scalar
    /// * `a`: Color reference
    pub fn multiply(s: f64, a: &Color) -> Color {
        Color {
            r: s * a.r,
            g: s * a.g,
            b: s * a.b,
        }
    }

    /// divide a color by a scalar returning a new color instance
    /// #### Arguments
    /// * `a`: Color reference
    /// * `s`: scalar
    pub fn divide(a: &Color, s: f64) -> Color {
        Color {
            r: a.r / s,
            g: a.g / s,
            b: a.b / s,
        }
    }

    /// clamp color values between 0 and 1 returning a new color instance
    /// #### Arguments
    /// * `v`: Color reference
    pub fn clamp(v: &Color) -> Color {
        Color {
            r: v.r.min(1.0).max(0.0),
            g: v.g.min(1.0).max(0.0),
            b: v.b.min(1.0).max(0.0),
        }
    }
}