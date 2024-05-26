use crate::create_intersections;
use crate::math::{utils, Color, Matrix, Vector};

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
    pub t: f64,
    pub obj: Rc<RefCell<T>>,
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

/// Point Light Source with no size
pub struct PointLight {
    pos: Vector,
    int: Color, // intensity
}

impl PointLight {
    pub fn new(pos: Vector, int: Color) -> Self {
        Self { pos, int }
    }

    /// Shades individual world pixels based on the Phong Reflection Model, returning Color value at the point.
    /// m - Material of the object where the world pixel belongs to;
    /// p - The position of the point;
    /// e - Eye vector of the camera;
    /// n - Normal to the object at the world pixel;
    pub fn shade(&self, m: &Material, p: &Vector, e: &Vector, n: &Vector) -> Color {
        // combine the surface color with the light's intensity
        let eff_col = self.int * m.color; // effective color

        // find the direction to the light source
        let l = (self.pos - p).normalize();

        // compute the ambient contribution
        let ambient = eff_col * m.ambient;

        let diffuse: Color;
        let specular: Color;

        // ldn represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means
        // the light is on the other side of the surface
        let ldn = utils::dot(&l, n); // light_dot_normal
        if ldn < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // compute the diffusion contribution
            diffuse = eff_col * m.diffuse * ldn;

            // reflected light vector
            let r = utils::reflect(&(-l), n);

            // rde represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let rde = utils::dot(&r, e); // reflect_dot_eye
            if rde <= 0.0 {
                specular = Color::black();
            } else {
                // compute the specular contribution
                let factor = rde.powf(m.shininess);
                specular = self.int * m.specular * factor;
            }
        }

        let res = ambient + specular + diffuse;
        return res;
    }
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            pos: utils::point(0.0, 0.0, 0.0),
            int: utils::color(1.0, 1.0, 1.0),
        }
    }
}

/// Phong Reflection Model's material data structure
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: utils::color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}
