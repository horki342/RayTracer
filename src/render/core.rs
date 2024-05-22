use crate::create_intersections;
use crate::math::{utils, Matrix, Vector};

use super::shapes::{Drawable, Sphere};

use std::cell::RefCell;
use std::ops;
use std::{fmt::Debug, rc::Rc};

/// Structure that implements Ray
#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    /// Creates an instance of Ray
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    /// Get position at t
    pub fn pos(&self, t: f64) -> Vector {
        return self.origin + self.direction * t;
    }

    /// Intersect the sphere
    pub fn intersect_sphere(&self, sphere: Rc<RefCell<Sphere>>) -> Intersections<Sphere> {
        let s = sphere.borrow_mut();

        // inversed Sphere's transformation matrix
        let st_inv =
            s.t.inverse()
                .expect("Could not invert the Sphere's Transformation");

        // apply the inverse of sphere's transformation onto the ray
        let r = &st_inv * self;

        let del = r.origin - s.c;

        let a = utils::dot(&del, &del) - s.r;
        let b = utils::dot(&r.direction, &del);
        let c = utils::dot(&r.direction, &r.direction);

        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            create_intersections!()
        } else {
            let t1 = (-b + discriminant.sqrt()) / c;
            let t2 = (-b - discriminant.sqrt()) / c;

            create_intersections!(
                t1, sphere.clone();
                t2, sphere.clone();
            )
        }
    }
}

impl ops::Mul<&Ray> for &Matrix {
    type Output = Ray;

    /// Applies matrix transformation to a ray
    fn mul(self, rhs: &Ray) -> Self::Output {
        return Ray::new(self * rhs.origin, self * rhs.direction);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<T: Drawable> {
    t: f64,
    obj: Rc<RefCell<T>>,
}

impl<T: Drawable> Intersection<T> {
    pub fn new(t: f64, obj: Rc<RefCell<T>>) -> Self {
        Self { t, obj }
    }
}

#[derive(Debug, Clone)]
pub struct Intersections<T: Drawable> {
    data: Vec<Intersection<T>>,
}

impl<T: Drawable> Intersections<T> {
    /// Creates Intersections and sort the data
    pub fn from(data: Vec<Intersection<T>>) -> Self {
        let mut res = Intersections { data };
        res.sort();
        res
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn contains(&self, val: f64) -> bool {
        for intersection in &self.data {
            if utils::feq(intersection.t, val) {
                return true;
            }
        }
        return false;
    }

    pub fn sort(&mut self) {
        self.data
            .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
    }

    pub fn hit(&self) -> Option<&Intersection<T>> {
        for el in &self.data {
            if el.t < 0.0 {
                continue;
            }
            return Some(el);
        }

        return None;
    }
}

impl<T: Drawable> Default for Intersections<T> {
    fn default() -> Self {
        Self { data: vec![] }
    }
}

#[macro_export]
macro_rules! create_intersections {
    () => {
        crate::render::core::Intersections::from(Vec::new())
    };

    ($($i:expr),* $(,)?) => {{
        let mut vec = Vec::new();
        $(
            vec.push($i);
        )*
        crate::render::core::Intersections::from(vec)
    }};

    ($t:expr, $obj:expr; $($rest:tt)*) => {{
        let mut vec = create_intersections!($($rest)*).data;
        vec.push(Intersection::new($t, $obj));
        Intersections::from(vec)
    }};
}
