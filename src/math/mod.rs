use nalgebra as na;
use std::ops;

use super::render::core::Ray;

pub mod utils;

/// The only Vector-type used in the application.
pub type Vector = na::Vector4<f64>;

/// The only Matrix-type used in the application
pub type Matrix = na::Matrix4<f64>;

/// Structure that implements Color in RGB-format
/// r: red-value in the range 0..1
/// g: green-value in the range 0..1
/// b: blue-value in the range 0..1
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    /// Creates a new Color with specified f64 RGB values (r, g, b)
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    /// Converts val from 0..1 (like r, g, b of Color) to 0..255 (used in PPM-loading)
    fn cvt(val: f64) -> i32 {
        if val > 1.0 {
            return 255;
        }
        if val < 0.0 {
            return 0;
        }

        (val * 255.0).round() as i32
    }

    /// formats Color-type for printing/debugging
    pub fn fmt(&self) -> String {
        format!(
            "{} {} {}",
            Color::cvt(self.r),
            Color::cvt(self.g),
            Color::cvt(self.b),
        )
    }
}
impl Default for Color {
    /// return a default black color
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}
impl PartialEq for Color {
    /// Compares two Color-types
    fn eq(&self, other: &Self) -> bool {
        utils::feq(self.r, other.r) && utils::feq(self.g, other.g) && utils::feq(self.b, other.b)
    }
}
impl ops::Add<Color> for Color {
    type Output = Color;

    /// Adds two colors component-wisely
    fn add(self, rhs: Color) -> Self::Output {
        utils::color(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}
impl ops::Sub<Color> for Color {
    type Output = Color;

    /// Subtracts two colors component-wisely
    fn sub(self, rhs: Color) -> Self::Output {
        utils::color(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}
impl ops::Mul<f64> for Color {
    type Output = Color;

    /// Multiplies a Color with a float number
    fn mul(self, rhs: f64) -> Self::Output {
        utils::color(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}
impl ops::Mul<Color> for f64 {
    type Output = Color;

    /// Multiplies a Color with a float number
    fn mul(self, rhs: Color) -> Self::Output {
        utils::color(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}
impl ops::Div<f64> for Color {
    type Output = Color;

    /// Divides a Color with a float number
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl ops::Mul<Color> for Color {
    type Output = Color;

    /// Schur-product of two colors
    fn mul(self, rhs: Color) -> Self::Output {
        utils::color(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

/// Enumeration that holds a convenient way of indicating single transformation units, and provides a framework for getting matrices.
#[derive(Debug, Clone, PartialEq)]
pub enum TUnit {
    /// Translation(dx, dy, dz) represents a translation from the origin to (x0 + dx, y0 + dy, z0 + dz);
    Translate(f64, f64, f64),

    /// Scale(fx, fy, fz) represents scaling in correponding dimensions (fx, fy, fz);
    Scale(f64, f64, f64),

    /// RotateX(angle) represents a rotation around x-axis (left-hand rule) at angle radians;
    RotateX(f64),

    /// RotateY(angle) represents a rotation around y-axis (left-hand rule) at angle radians;
    RotateY(f64),

    /// RotateZ(angle) represents a rotation around z-axis (left-hand rule) at angle radians;
    RotateZ(f64),

    /// Shear(xy, xz, yx, yz, zx, zy) represents shearing in 6 subspaces
    Shear(f64, f64, f64, f64, f64, f64),

    /// None represents that no transformation is applied (corresponds to I)
    None,
}

impl TUnit {
    /// returns a translation matrix
    fn translate_matrix(dx: f64, dy: f64, dz: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_column(3, &Vector::new(dx, dy, dz, 1.0));
        res
    }

    /// returns a scale matrix
    fn scale_matrix(fx: f64, fy: f64, fz: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_diagonal(&Vector::new(fx, fy, fz, 1.0));
        res
    }

    /// returns a rotation around x-axis matrix
    fn rotate_x_matrix(angle: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_column(1, &Vector::new(0.0, angle.cos(), angle.sin(), 0.0));
        res.set_column(2, &Vector::new(0.0, -angle.sin(), angle.cos(), 0.0));

        res
    }

    /// returns a rotation around y-axis matrix
    fn rotate_y_matrix(angle: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_column(0, &Vector::new(angle.cos(), 0.0, -angle.sin(), 0.0));
        res.set_column(2, &Vector::new(angle.sin(), 0.0, angle.cos(), 0.0));

        res
    }

    /// returns a rotation around z-axis matrix
    fn rotate_z_matrix(angle: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_column(0, &Vector::new(angle.cos(), angle.sin(), 0.0, 0.0));
        res.set_column(1, &Vector::new(-angle.sin(), angle.cos(), 0.0, 0.0));

        res
    }

    /// returns a shear matrix
    fn shear_matrix(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_column(0, &Vector::new(1.0, yx, zx, 0.0));
        res.set_column(1, &Vector::new(xy, 1.0, zy, 0.0));
        res.set_column(2, &Vector::new(xz, yz, 1.0, 0.0));

        res
    }

    /// returns a matrix-form of the TUnit (given as &self)
    pub fn matrix(&self) -> Matrix {
        match self {
            TUnit::None => Matrix::identity(),
            TUnit::Translate(dx, dy, dz) => TUnit::translate_matrix(*dx, *dy, *dz),
            TUnit::Scale(fx, fy, fz) => TUnit::scale_matrix(*fx, *fy, *fz),
            TUnit::RotateX(angle) => TUnit::rotate_x_matrix(*angle),
            TUnit::RotateY(angle) => TUnit::rotate_y_matrix(*angle),
            TUnit::RotateZ(angle) => TUnit::rotate_z_matrix(*angle),
            TUnit::Shear(xy, xz, yx, yz, zx, zy) => {
                TUnit::shear_matrix(*xy, *xz, *yx, *yz, *zx, *zy)
            }
        }
    }
}

impl ops::Mul<Vector> for &TUnit {
    type Output = Vector;

    /// Applies a single tranformation unit (TUnit) to a vector via Matrix-Vector multiplication
    fn mul(self, rhs: Vector) -> Self::Output {
        return self.matrix() * rhs;
    }
}

impl ops::Mul<Ray> for &TUnit {
    type Output = Ray;

    /// Applies a single transformation unit (TUnit) to a ray
    fn mul(self, rhs: Ray) -> Self::Output {
        let mat = self.matrix();

        return Ray::new(&mat * rhs.origin, &mat * rhs.direction);
    }
}

/// A data structure that represents an ordered set of TUnits (individual transformations). It stores the matrix form that must be computed once per set. However, when a new TUnit is added, new matrix is calculated by multiplying the old one with the new TUnit-matrix. On the other hand, whenever a TUnit is removed, the matrix must be re-computed.
#[derive(Debug, Clone, PartialEq)]
pub struct Transformation {
    set: Vec<TUnit>,
    matrix: Matrix,
}

impl Transformation {
    /// Creates a new Transformation structure with provided TUnits via the slice
    pub fn new(set: &[TUnit]) -> Self {
        let mut res = Self {
            set: set.to_vec(),
            matrix: Matrix::identity(),
        };

        res.adjust_matrix(0);
        return res;
    }

    /// Adjusts the matrix of Transformation. User must not call this function manually
    /// start_idx: index to start adjusting from
    fn adjust_matrix(&mut self, start_idx: usize) {
        for el in &self.set[start_idx..] {
            self.matrix = el.matrix() * self.matrix;
        }
    }

    /// Add a TUnit to existing transformation set, and adjust the matrix.
    pub fn add(&mut self, transformation: TUnit) {
        self.set.push(transformation);
        self.adjust_matrix(self.set.len() - 1);
    }

    /// Add several TUnits to the transformation set, and adjust the matrix.
    pub fn add_several(&mut self, transformations: &[TUnit]) {
        self.set.extend_from_slice(transformations);
        self.adjust_matrix(self.set.len() - transformations.len());
    }

    /// Returns a non-mutable reference to the matrix of the Transformation object
    pub fn matrix(&self) -> &Matrix {
        return &self.matrix;
    }

    /// Returns an inverse matrix (ownership)
    pub fn inverse(&self) -> Option<Matrix> {
        self.matrix.try_inverse()
    }
}

impl ops::Mul<Vector> for &Transformation {
    type Output = Vector;

    /// Multiply Transformation.matrix() with the Vector type
    fn mul(self, rhs: Vector) -> Self::Output {
        return self.matrix() * rhs;
    }
}

impl Default for Transformation {
    /// Creates a new empty Transformation with set{ vec![], Matrix::identity() }
    fn default() -> Self {
        Self {
            set: vec![],
            matrix: Matrix::identity(),
        }
    }
}
