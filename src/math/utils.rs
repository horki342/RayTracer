use super::{Color, Matrix, Vector};
use nalgebra::Vector3;

pub const EPSILON: f64 = 0.0001;

/// Compares two float (f64) numbers with EPSILON-precision
pub fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

/// Compares two vector-types (Vector) with EPSILON-precision
pub fn veq(a: &Vector, b: &Vector) -> bool {
    (a - b).norm() < EPSILON
}

/// Compares two matrix-types (Matrix) with EPSILON-precision
pub fn meq(a: &Matrix, b: &Matrix) -> bool {
    (a - b).norm() < EPSILON
}

#[macro_export]
/// Utility that expand vassert!(Vector, Vector) to assert!(veq(&Vector, &Vector))
macro_rules! vassert {
    ($v1:expr, $v2:expr) => {
        assert!(crate::math::utils::veq(&$v1, &$v2));
    };
}

#[macro_export]
/// Utility that expand fassert!(f64, f64) to assert!(feq(f64, f64))
macro_rules! fassert {
    ($f1:expr, $f2:expr) => {
        assert!(crate::math::utils::feq($f1, $f2));
    };
}

#[macro_export]
/// Utility that expand massert!(Matrix, Matrix) to assert!(meq(&Matrix, &Matrix))
macro_rules! massert {
    ($m1:expr, $m2:expr) => {
        assert!(crate::math::utils::meq(&$m1, &$m2))
    };
}

/// Returns a color with a specified RGB-color
pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color::new(r, g, b)
}

/// Returns a Vector::new(x, y, z, w) with specified four f64-numbers
pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Vector {
    Vector::new(x, y, z, w)
}

/// Returns a point(x, y, z), which is a tuple with w set to 1.0
pub fn point(x: f64, y: f64, z: f64) -> Vector {
    tuple(x, y, z, 1.0)
}

/// Returns a vector(x, y, z), which is a tuple with w set to 0.0
pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    tuple(x, y, z, 0.0)
}

/// Returns a 4 by 4 matrix, which is of type Matrix = na::Matrix4
pub fn matrix(
    a11: f64,
    a12: f64,
    a13: f64,
    a14: f64,
    a21: f64,
    a22: f64,
    a23: f64,
    a24: f64,
    a31: f64,
    a32: f64,
    a33: f64,
    a34: f64,
    a41: f64,
    a42: f64,
    a43: f64,
    a44: f64,
) -> Matrix {
    Matrix::new(
        a11, a12, a13, a14, a21, a22, a23, a24, a31, a32, a33, a34, a41, a42, a43, a44,
    )
}

/// Returns a dot product of two vectors
pub fn dot(a: &Vector, b: &Vector) -> f64 {
    return a.dot(&b);
}

/// Returns a cross product of two Vector types.
pub fn cross(a: &Vector, b: &Vector) -> Vector {
    let a_ = Vector3::<f64>::new(a.x, a.y, a.z);
    let b_ = Vector3::<f64>::new(b.x, b.y, b.z);
    let res = a_.cross(&b_);

    vector(res.x, res.y, res.z)
}

#[macro_export]
/// A macro that provides a convenient interface for creating Transformation objects.
macro_rules! transform {
    () => {
        Transformation::default()
    };

    ( $( $x:expr ),* ) => {
        {
            let mut result = Transformation::default();
            $(
                let temp = $x;
                result.add(temp);
            )*
            result
        }
    };
}

/// reflects a vector across a (normal) vector
pub fn reflect(v: &Vector, n: &Vector) -> Vector {
    return v - 2.0 * n * dot(n, v);
}
