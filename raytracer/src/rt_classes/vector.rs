use std::ops::{Add, Sub, Mul, Neg, Div};

#[derive(Debug, Copy, Clone)]
pub struct Direction {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[allow(dead_code)]
impl Direction {
    pub fn new(x: f64, y: f64, z: f64) -> Direction {
        Direction { x, y, z }
    }
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}

#[allow(dead_code)]
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }
}

#[allow(dead_code)]
pub struct Vector;

#[allow(dead_code)]
impl Vector {
    pub fn direction(x: f64, y: f64, z: f64) -> Direction {
        Direction::new(x, y, z)
    }

    pub fn point(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    pub fn color(r: f64, g: f64, b: f64) -> Color {
        Color::new(r, g, b)
    }

    pub fn add_direction(a: Direction, b: Direction) -> Direction {
        let result = a + b;
        result
    }

    pub fn add_point(a: Point, b: Point) -> Point {
        let result = a + b;
        result
    }

    pub fn add_point_direction(a: Point, b: Direction) -> Point {
        let result = Point::new(a.x + b.x, a.y + b.y, a.z + b.z);
        result
    }

    pub fn subtract_direction(a: Direction, b: Direction) -> Direction {
        let result = a - b;
        result
    }

    pub fn subtract_point(a: Point, b: Point) -> Direction {
        Direction::new(a.x - b.x, a.y - b.y, a.z - b.z)
    }

    pub fn subtract_point_direction(a: Point, b: Direction) -> Point {
        Point::new(a.x - b.x, a.y - b.y, a.z - b.z)
    }

    pub fn interpolate(a: Point, b: Point, t: f64) -> Point {
        Point::new(
            a.x * (1.0 - t) + b.x * t,
            a.y * (1.0 - t) + b.y * t,
            a.z * (1.0 - t) + b.z * t,
        )
    }

    pub fn multiply_scalar_direction(s: f64, a: Direction) -> Direction {
        Direction::new(s * a.x, s * a.y, s * a.z)
    }

    pub fn multiply_scalar_point(s: f64, a: Point) -> Direction {
        Direction::new(s * a.x, s * a.y, s * a.z)
    }

    pub fn multiply_direction_scalar(a: Direction, s: f64) -> Direction {
        Direction::new(s * a.x, s * a.y, s * a.z)
    }

    pub fn negate_direction(a: Direction) -> Direction {
        Direction::new(-a.x, -a.y, -a.z)
    }

    pub fn divide_direction_scalar(a: Direction, s: f64) -> Direction {
        Direction::new(a.x / s, a.y / s, a.z / s)
    }

    pub fn divide_point_scalar(a: Point, s: f64) -> Direction {
        Direction::new(a.x / s, a.y / s, a.z / s)
    }

    pub fn dot_product_direction_direction(a: Direction, b: Direction) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn dot_product_direction_point(a: Direction, b: Point) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn dot_product_point_direction(a: Point, b: Direction) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn dot_product_point_point(a: Point, b: Point) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn dot_product_without_y_direction_direction(a: Direction, b: Direction) -> f64 {
        a.x * b.x + a.z * b.z
    }

    pub fn dot_product_without_y_direction_point(a: Direction, b: Point) -> f64 {
        a.x * b.x + a.z * b.z
    }

    pub fn dot_product_without_y_point_direction(a: Point, b: Direction) -> f64 {
        a.x * b.x + a.z * b.z
    }

    pub fn dot_product_without_y_point_point(a: Point, b: Point) -> f64 {
        a.x * b.x + a.z * b.z
    }

    pub fn cross_product(a: Direction, b: Direction) -> Direction {
        Direction::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn length(a: Direction) -> f64 {
        f64::sqrt(a.x * a.x + a.y * a.y + a.z * a.z)
    }

    pub fn squared_length(a: Direction) -> f64 {
        a.x * a.x + a.y * a.y + a.z * a.z
    }

    pub fn normalize(a: Direction) -> Direction {
        Vector::divide_direction_scalar(a, Vector::length(a))
    }

    pub const ZERO: Point = Point { x: 0.0, y: 0.0, z: 0.0 };
    pub const X_AXIS: Direction = Direction { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y_AXIS: Direction = Direction { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z_AXIS: Direction = Direction { x: 0.0, y: 0.0, z: 1.0 };
    pub const NX_AXIS: Direction = Direction { x: -1.0, y: 0.0, z: 0.0 };
    pub const NY_AXIS: Direction = Direction { x: 0.0, y: -1.0, z: 0.0 };
    pub const NZ_AXIS: Direction = Direction { x: 0.0, y: 0.0, z: -1.0 };
}

impl Add for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Direction {
        Direction::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Direction {
    type Output = Direction;

    fn sub(self, rhs: Direction) -> Direction {
        Direction::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Point> for Point {
    type Output = Direction;

    fn sub(self, rhs: Point) -> Direction {
        Direction::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        Direction::new(-self.x, -self.y, -self.z)
    }
}

impl Div<f64> for Direction {
    type Output = Direction;

    fn div(self, rhs: f64) -> Direction {
        Direction::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<f64> for Point {
    type Output = Direction;

    fn div(self, rhs: f64) -> Direction {
        Direction::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<f64> for Direction {
    type Output = Direction;

    fn mul(self, rhs: f64) -> Direction {
        Direction::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<f64> for Point {
    type Output = Direction;

    fn mul(self, rhs: f64) -> Direction {
        Direction::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}