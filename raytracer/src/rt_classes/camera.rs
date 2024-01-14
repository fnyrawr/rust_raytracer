use crate::rt_classes::matrix::Matrix;
use crate::rt_classes::vector::Point;

pub struct CameraObscura {
    pub phi: f64,
    pub position: Point,
    pub matrix: Matrix,
    pub width: f64,
    pub height: f64,
}

#[allow(dead_code)]
impl CameraObscura {
    pub fn new(phi: f64, position: Point, width: f64, height: f64) -> CameraObscura {
        CameraObscura { phi, position, matrix: Matrix::identity(), width, height }
    }

    pub fn new_with_matrix(phi: f64, position: Point, matrix: Matrix, width: f64, height: f64) -> CameraObscura {
        CameraObscura { phi, position, matrix, width, height }
    }
}