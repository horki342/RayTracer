use nalgebra as na;
use std::ops;

use crate::draw::shapes::Transformable;

const EPSILON: f64 = 0.0001;
pub type Vector = na::Vector4<f64>;
pub type Matrix = na::Matrix4<f64>;

/// Structure that implements Color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        feq(self.r, other.r) && feq(self.g, other.g) && feq(self.b, other.b)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    fn cvt(val: f64) -> i32 {
        if val > 1.0 {
            return 255;
        }
        if val < 0.0 {
            return 0;
        }

        (val * 255.0).round() as i32
    }

    pub fn fmt(&self) -> String {
        format!(
            "{} {} {}",
            Color::cvt(self.r),
            Color::cvt(self.g),
            Color::cvt(self.b)
        )
    }
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color::new(r, g, b)
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Vector {
    Vector::new(x, y, z, w)
}

pub fn point(x: f64, y: f64, z: f64) -> Vector {
    tuple(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    tuple(x, y, z, 0.0)
}

pub fn dot(a: &Vector, b: &Vector) -> f64 {
    return a.dot(&b);
}

pub fn cross(a: &Vector, b: &Vector) -> Vector {
    let a_ = na::Vector3::<f64>::new(a.x, a.y, a.z);
    let b_ = na::Vector3::<f64>::new(b.x, b.y, b.z);
    let res = a_.cross(&b_);

    vector(res.x, res.y, res.z)
}

pub fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn veq(a: &Vector, b: &Vector) -> bool {
    (a - b).norm() < EPSILON
}

pub fn meq(a: &Matrix, b: &Matrix) -> bool {
    (a - b).norm() < EPSILON
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pipeline {
    operations: Vec<Transformation>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            operations: vec![Transformation::None],
        }
    }
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            operations: Vec::new(),
        }
    }

    pub fn from(operations: Vec<Transformation>) -> Pipeline {
        Pipeline {
            operations: operations,
        }
    }

    pub fn get_matrix(&self) -> Matrix {
        let mut res = Matrix::identity();

        for operation in &self.operations {
            res = operation.get_matrix() * res;
        }

        res
    }

    pub fn add(&mut self, t: Transformation) {
        self.operations.push(t);
    }

    pub fn set_to<T: Transformable>(&self, obj: &mut T) {
        obj.set_transforms(self.to_owned());
    }

    pub fn apply_to<T: Transformable>(&self, obj: &mut T) {
        obj.set_transforms(self.to_owned());
        obj.transform_this();
    }

    pub fn applied<T: Transformable>(&self, obj: &mut T) -> T {
        obj.set_transforms(self.to_owned());
        return obj.transformed();
    }
}

#[macro_export]
macro_rules! transform {
    () => {
        transform!(Transformation::None)
    };

    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Pipeline::from(temp_vec)
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transformation {
    Translate(f64, f64, f64),
    Scale(f64, f64, f64),
    RotateX(f64),
    RotateY(f64),
    RotateZ(f64),
    Shear(f64, f64, f64, f64, f64, f64),
    None,
}

impl Transformation {
    fn translate(dx: f64, dy: f64, dz: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_column(3, &Vector::new(dx, dy, dz, 1.0));
        res
    }

    fn scale(fx: f64, fy: f64, fz: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_diagonal(&Vector::new(fx, fy, fz, 1.0));
        res
    }

    fn rotate_x(angle: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_column(1, &Vector::new(0.0, angle.cos(), angle.sin(), 0.0));
        res.set_column(2, &Vector::new(0.0, -angle.sin(), angle.cos(), 0.0));

        res
    }

    fn rotate_y(angle: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_column(0, &Vector::new(angle.cos(), 0.0, -angle.sin(), 0.0));
        res.set_column(2, &Vector::new(angle.sin(), 0.0, angle.cos(), 0.0));

        res
    }

    fn rotate_z(angle: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_column(0, &Vector::new(angle.cos(), angle.sin(), 0.0, 0.0));
        res.set_column(1, &Vector::new(-angle.sin(), angle.cos(), 0.0, 0.0));

        res
    }

    fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        let mut res = Matrix::identity();
        res.set_column(0, &Vector::new(1.0, yx, zx, 0.0));
        res.set_column(1, &Vector::new(xy, 1.0, zy, 0.0));
        res.set_column(2, &Vector::new(xz, yz, 1.0, 0.0));

        res
    }

    pub fn get_matrix(&self) -> Matrix {
        match self {
            Transformation::None => return Matrix::identity(),
            Transformation::Translate(dx, dy, dz) => {
                return Transformation::translate(*dx, *dy, *dz)
            }
            Transformation::Scale(fx, fy, fz) => return Transformation::scale(*fx, *fy, *fz),
            Transformation::RotateX(angle) => return Transformation::rotate_x(*angle),
            Transformation::RotateY(angle) => return Transformation::rotate_y(*angle),
            Transformation::RotateZ(angle) => return Transformation::rotate_z(*angle),
            Transformation::Shear(xy, xz, yx, yz, zx, zy) => {
                return Transformation::shear(*xy, *xz, *yx, *yz, *zx, *zy)
            }
        }
    }
}
