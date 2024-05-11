use super::equal;
use std::ops;

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
