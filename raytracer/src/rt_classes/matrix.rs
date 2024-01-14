use std::ops::Mul;

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

pub struct Matrix {
    values: [f64; 16],
}

#[allow(dead_code)]
impl Matrix {
    pub fn matrix(b0: Direction, b1: Direction, b2: Direction) -> Matrix {
        let mut m = Matrix::identity();
        m.set(0, 0, b0.x);
        m.set(1, 0, b0.y);
        m.set(2, 0, b0.z);
        m.set(0, 1, b1.x);
        m.set(1, 1, b1.y);
        m.set(2, 1, b1.z);
        m.set(0, 2, b2.x);
        m.set(1, 2, b2.y);
        m.set(2, 2, b2.z);
        m
    }

    pub fn matrix_with_point(b0: Direction, b1: Direction, b2: Direction, b3: Point) -> Matrix {
        let mut m = Matrix::matrix(b0, b1, b2);
        m.set(0, 3, b3.x);
        m.set(1, 3, b3.y);
        m.set(2, 3, b3.z);
        m
    }

    pub fn identity() -> Matrix {
        Matrix {
            values: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn translation<T: Into<Direction>>(t: T) -> Matrix {
        let t = t.into();
        let mut m = Matrix::identity();
        m.set(3, 0, t.x);
        m.set(3, 1, t.y);
        m.set(3, 2, t.z);
        m
    }

    pub fn translation_point<T: Into<Point>>(t: T) -> Matrix {
        let t = t.into();
        let mut m = Matrix::identity();
        m.set(3, 0, t.x);
        m.set(3, 1, t.y);
        m.set(3, 2, t.z);
        m
    }

    pub fn translation_xyz(x: f64, y: f64, z: f64) -> Matrix {
        let mut m = Matrix::identity();
        m.set(3, 0, x);
        m.set(3, 1, y);
        m.set(3, 2, z);
        m
    }

    pub fn rotation(axis: Direction, angle: f64) -> Matrix {
        let mut m = Matrix::identity();
        let rad = (angle / 180.0) * std::f64::consts::PI;
        let cosa = f64::cos(rad);
        let sina = f64::sin(rad);
        let l = f64::sqrt(axis.x * axis.x + axis.y * axis.y + axis.z * axis.z);
        let rx = axis.x / l;
        let ry = axis.y / l;
        let rz = axis.z / l;
        let icosa = 1.0 - cosa;

        m.set(0, 0, icosa * rx * rx + cosa);
        m.set(0, 1, icosa * rx * ry + rz * sina);
        m.set(0, 2, icosa * rx * rz - ry * sina);

        m.set(1, 0, icosa * rx * ry - rz * sina);
        m.set(1, 1, icosa * ry * ry + cosa);
        m.set(1, 2, icosa * ry * rz + rx * sina);

        m.set(2, 0, icosa * rx * rz + ry * sina);
        m.set(2, 1, icosa * ry * rz - rx * sina);
        m.set(2, 2, icosa * rz * rz + cosa);

        m
    }

    pub fn rotation_xyz(ax: f64, ay: f64, az: f64, angle: f64) -> Matrix {
        Matrix::rotation(Direction { x: ax, y: ay, z: az }, angle)
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let mut m = Matrix::identity();
        m.set(0, 0, x);
        m.set(1, 1, y);
        m.set(2, 2, z);
        m
    }

    pub fn multiply(a: Matrix, b: Matrix) -> Matrix {
        let r = a * b;
        r
    }

    pub fn multiply_point(m: Matrix, p: Point) -> Point {
        Point {
            x: m.get(0, 0) * p.x + m.get(1, 0) * p.y + m.get(2, 0) * p.z + m.get(3, 0),
            y: m.get(0, 1) * p.x + m.get(1, 1) * p.y + m.get(2, 1) * p.z + m.get(3, 1),
            z: m.get(0, 2) * p.x + m.get(1, 2) * p.y + m.get(2, 2) * p.z + m.get(3, 2),
        }
    }

    pub fn multiply_direction(m: Matrix, d: Direction) -> Direction {
        Direction {
            x: m.get(0, 0) * d.x + m.get(1, 0) * d.y + m.get(2, 0) * d.z,
            y: m.get(0, 1) * d.x + m.get(1, 1) * d.y + m.get(2, 1) * d.z,
            z: m.get(0, 2) * d.x + m.get(1, 2) * d.y + m.get(2, 2) * d.z,
        }
    }

    pub fn transpose(m: Matrix) -> Matrix {
        let mut n = Matrix::identity();
        for c in 0..4 {
            for r in 0..4 {
                n.set(c, r, m.get(r, c));
            }
        }
        n
    }

    pub fn inverse(m: Matrix) -> Matrix {
        let mut inv = Matrix::identity();
        let det = m.determinant();

        if det != 0.0 {
            let inv_det = 1.0 / det;

            for col in 0..4 {
                for row in 0..4 {
                    let cofactor = m.cofactor(row, col);
                    inv.set(row, col, cofactor * inv_det);
                }
            }
        }

        inv
    }

    fn determinant(&self) -> f64 {
        self.get(0, 0) * self.minor(1, 2, 3, 1, 2, 3)
            - self.get(0, 1) * self.minor(1, 2, 3, 0, 2, 3)
            + self.get(0, 2) * self.minor(1, 2, 3, 0, 1, 3)
            - self.get(0, 3) * self.minor(1, 2, 3, 0, 1, 2)
    }

    fn minor(&self, r0: usize, r1: usize, r2: usize, c0: usize, c1: usize, c2: usize) -> f64 {
        self.get(r0, c0) * (self.get(r1, c1) * self.get(r2, c2) - self.get(r2, c1) * self.get(r1, c2))
            - self.get(r0, c1) * (self.get(r1, c0) * self.get(r2, c2) - self.get(r2, c0) * self.get(r1, c2))
            + self.get(r0, c2) * (self.get(r1, c0) * self.get(r2, c1) - self.get(r2, c0) * self.get(r1, c1))
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
        sign * self.minor((row + 1) % 4, (row + 2) % 4, (row + 3) % 4, (col + 1) % 4, (col + 2) % 4, (col + 3) % 4)
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        self.values[row * 4 + col] = value;
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        self.values[row * 4 + col]
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix::identity();

        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.get(i, k) * rhs.get(k, j);
                }
                result.set(i, j, sum);
            }
        }

        result
    }
}