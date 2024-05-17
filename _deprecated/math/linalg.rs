use super::equal;
use core::fmt;
use std::ops;

/// data structure that implements matrix functionality
#[derive(Debug)]
pub struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<T>,
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            write!(f, "| ")?;
            for j in 0..self.width {
                write!(f, "{:4.1} | ", self.grid[self.width * i + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub trait MatrixMethods<T: Copy + Default + Clone> {
    fn new_with_value(width: usize, height: usize, val: T) -> Self;
    fn new(width: usize, height: usize) -> Self;
    fn new_with_values(width: usize, height: usize, args: &[T]) -> Self;

    /// writes value in a given location
    fn write(&mut self, col: usize, row: usize, val: T);

    /// return a value in a given location
    fn at(&self, col: usize, row: usize) -> &T;

    /// write lots of valuesin the format [..row1 ..row2 ..row3 ..etc]
    fn fill(&mut self, args: &[T]);

    fn transpose(&self) -> Self;

    /// Returns a submatrix by removing the specified row and column
    fn submatrix(&self, r: usize, c: usize) -> Self;

    /// Returns the determinant of the matrix
    fn det(&self) -> T;

    /// Returns the minor of the matrix at the specified row and column
    fn minor(&self, r: usize, c: usize) -> T;

    /// Returns the cofactor of the matrix at the specified row and column
    fn cofactor(&self, r: usize, c: usize) -> T;
}

// returns a default 4 by 4 matrix (zero in case of f32 and i32)
impl<T: Clone + Default> Default for Matrix<T> {
    fn default() -> Self {
        Self {
            width: 4_usize,
            height: 4_usize,
            grid: vec![T::default(); 16_usize],
        }
    }
}

impl<T> ops::Index<[usize; 2]> for Matrix<T> {
    type Output = T;

    /// indices contains: [row, col]
    fn index(&self, indices: [usize; 2]) -> &Self::Output {
        return &self.grid[self.width * indices[0] + indices[1]];
    }
}

impl<T> ops::IndexMut<[usize; 2]> for Matrix<T> {
    /// indices contains: [row, col]
    fn index_mut(&mut self, indices: [usize; 2]) -> &mut Self::Output {
        return &mut self.grid[self.width * indices[0] + indices[1]];
    }
}

/// returns a 4 by 4 f32 matrix with provided args (the most common matrix used)
pub fn mat4(args: &[f32]) -> Matrix<f32> {
    let res: Matrix<f32> = Matrix::new_with_values(4_usize, 4_usize, args);
    res
}

impl<T> MatrixMethods<T> for Matrix<T>
where
    T: Clone + Default + Copy + ops::Add<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T>,
{
    fn new(width: usize, height: usize) -> Matrix<T> {
        Matrix {
            width: width,
            height: height,
            grid: vec![T::default(); width * height],
        }
    }

    fn new_with_values(width: usize, height: usize, args: &[T]) -> Matrix<T> {
        Matrix {
            width: width,
            height: height,
            grid: args.to_vec(),
        }
    }

    fn new_with_value(width: usize, height: usize, val: T) -> Matrix<T> {
        Matrix {
            width: width,
            height: height,
            grid: vec![val; width * height],
        }
    }

    fn write(&mut self, col: usize, row: usize, val: T) {
        self.grid[self.width * col + row] = val;
    }

    fn at(&self, col: usize, row: usize) -> &T {
        &self.grid[self.width * col + row]
    }

    fn fill(&mut self, args: &[T]) {
        if self.width * self.height != args.len() {
            panic!("width * height of the matrix is not equal to a folded vector form");
        }

        for i in 0..self.height {
            for j in 0..self.width {
                self.write(j, i, args[self.width * i + j]);
            }
        }
    }

    fn transpose(&self) -> Self {
        let mut transposed = Matrix::new(self.height, self.width);
        for i in 0..self.width {
            for j in 0..self.height {
                transposed.write(i, j, self.at(j, i).clone());
            }
        }

        transposed
    }

    fn submatrix(&self, r: usize, c: usize) -> Self {
        // Check whether r and c are within boundaries
        if !(0..self.height).contains(&r) || !(0..self.width).contains(&c) {
            panic!("cannot delete a row/col from matrix");
        }

        let mut submatrix = Matrix::new(self.width - 1, self.height - 1);
        let mut sub_i = 0;

        for i in 0..self.height {
            if i == r {
                continue;
            }

            let mut sub_j = 0;
            for j in 0..self.width {
                if j == c {
                    continue;
                }

                submatrix.write(sub_j, sub_i, self[[i, j]].clone());
                sub_j += 1;
            }
            sub_i += 1;
        }

        submatrix
    }

    fn det(&self) -> T {
        if self.width != self.height {
            panic!("determinant is only defined for square matrices");
        }

        if self.width == 2 {
            // Base case for 2x2 matrix
            return self[[0, 0]] * self[[1, 1]] - self[[0, 1]] * self[[1, 0]];
        }

        let mut det = T::default();
        for c in 0..self.width {
            det = det + self[[0, c]] * self.cofactor(0, c);
        }
        det
    }

    fn minor(&self, r: usize, c: usize) -> T {
        self.submatrix(r, c).det()
    }

    fn cofactor(&self, r: usize, c: usize) -> T {
        let minor = self.minor(r, c);
        if (r + c) % 2 == 0 {
            return minor;
        } else {
            return T::default() - minor;
        }
    }
}

/// returns an identity matrix (works only for f32)
pub fn iden(width: usize, height: usize) -> Matrix<f32> {
    let mut res: Matrix<f32> = Matrix::new(width, height);

    for count in 0..(width * height) {
        if count % (width + 1) == 0 {
            res.grid[count] = 1.0;
        }
    }

    res
}

/// return a 4 by 4 identity matrix (works only for f32)
pub fn iden4() -> Matrix<f32> {
    iden(4_usize, 4_usize)
}

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut res: bool = self.width == other.width && self.height == other.height;

        // if matrices are of different size, they are not equal
        if !res {
            return false;
        }

        // check elements equality
        for i in 0..self.height {
            for j in 0..self.width {
                let idx = i * self.width + j;
                res = res && (self.grid[idx] == other.grid[idx]);
            }
        }

        res
    }
}

/// matrix-matrix multiplication
impl<T> ops::Mul<Matrix<T>> for Matrix<T>
where
    T: Default + Copy + Clone + ops::Add<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        // check whether two matrices may be multiplied
        if self.width != rhs.height {
            panic!("Could not multiply matrices: wrong dimensions");
        }

        let res_h = self.height;
        let res_w = rhs.width;
        let mut res: Matrix<T> = Matrix::new_with_value(res_w, res_h, T::default());

        for i in 0..res_h {
            for j in 0..res_w {
                for k in 0..self.width {
                    res[[i, j]] = res[[i, j]] + self[[i, k]] * rhs[[k, j]];
                }
            }
        }

        res
    }
}

/// matrix-tuple multiplication
impl ops::Mul<Tuple> for Matrix<f32> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        if self.width != self.height && self.width != 4_usize {
            panic!("only 4 by 4 matrix can multiply tuple (in current version)");
        }

        Tuple {
            x: rhs.x * self[[0, 0]]
                + rhs.y * self[[0, 1]]
                + rhs.z * self[[0, 2]]
                + rhs.w * self[[0, 3]],
            y: rhs.x * self[[1, 0]]
                + rhs.y * self[[1, 1]]
                + rhs.z * self[[1, 2]]
                + rhs.w * self[[1, 3]],
            z: rhs.x * self[[2, 0]]
                + rhs.y * self[[2, 1]]
                + rhs.z * self[[2, 2]]
                + rhs.w * self[[2, 3]],
            w: rhs.x * self[[3, 0]]
                + rhs.y * self[[3, 1]]
                + rhs.z * self[[3, 2]]
                + rhs.w * self[[3, 3]],
        }
    }
}

/// basic float32 number collection that represent a vector in four-dimensional space
#[derive(Debug)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// return point which is a Tuple(x, y, z, 1)
#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        crate::math::linalg::Tuple {
            x: $x,
            y: $y,
            z: $z,
            w: 1.0,
        }
    };
}

/// return vector which is a Tuple(x, y, z, 0)
#[macro_export]
macro_rules! vector {
    ($x:expr, $y:expr, $z:expr) => {
        crate::math::linalg::Tuple {
            x: $x,
            y: $y,
            z: $z,
            w: 0.0,
        }
    };
}

/// return tuple
#[macro_export]
macro_rules! tuple {
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        crate::math::linalg::Tuple {
            x: $x,
            y: $y,
            z: $z,
            w: $w,
        }
    };
}

/// returns a dot product of a and b
pub fn dot(a: &Tuple, b: &Tuple) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

/// returns a cross product for (ONLY) three dimensional vectors a and b
pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    // a and b must be vectors (w = 0)
    if equal(a.w, 0.0) || equal(b.w, 0.0) {
        println!("a and b both must be vectors");
    }

    vector![
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x
    ]
}

impl Tuple {
    /// returns a magnitude of the tuple
    pub fn mag(&self) -> f32 {
        let temp = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        if temp < 0.0 {
            panic!("Could not take sqrtf32 from a negative number");
        }
        temp.sqrt()
    }

    /// returns a norm of the tuple
    pub fn norm(self) -> Self {
        let magnitude = self.mag();
        self / magnitude
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        return equal(self.x, other.x)
            && equal(self.y, other.y)
            && equal(self.z, other.z)
            && equal(self.w, other.w);
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Self;

    /// adds two tuples
    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Self;

    /// substract two tuples
    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    /// negates a tuple
    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Mul<f32> for Tuple {
    type Output = Self;

    /// multiplies a tuple with a number
    fn mul(self, rhs: f32) -> Self::Output {
        tuple![self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs]
    }
}

impl ops::Mul<Tuple> for f32 {
    type Output = Tuple;

    /// multiplies a number with a tuple
    fn mul(self, rhs: Tuple) -> Self::Output {
        tuple![self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w]
    }
}

impl ops::Div<f32> for Tuple {
    type Output = Tuple;

    /// divides a tuple with a number
    fn div(self, rhs: f32) -> Self::Output {
        if equal(0.0, rhs) {
            panic!("Zero-division error (Tuple / f32)");
        }

        // faster than four different divisions
        return self * (1.0 / rhs);
    }
}
