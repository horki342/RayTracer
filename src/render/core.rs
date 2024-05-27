//! Contains the most important parts of the Ray Tracer,
//! and abstractions, like Shape and Drawable, that
//! may be inherited by specific shapes and be drawn onto the canvas

use std::fmt::Debug;
use std::rc::Rc;
use std::{cell::RefCell, ops};

use super::Canvas;
use crate::{
    math::{utils, Color, Matrix, TUnit, Transformation, Vector},
    transform,
};

/// Data structure that represents Phong's Reflection Model Material
/// ambient: Ambient lighting coefficient
/// diffuse: Diffuse lighting coefficient
/// specular: Specular lighting coefficient
/// shininess: Represents the shininess of the Light's reflection on the surface
/// color: Reflected Spectrum of light form object's surface (aka Color)
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    /// Change the color of the Material
    pub fn change_color(&mut self, col: Color) {
        self.color = col;
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

/// Data structure that represents Ray,
/// which is a working horse of the Ray Tracing Algorithm
/// origin: world-coordinate position of the ray
/// direction: direction of the ray
#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    /// Creates a new Ray with specified origin and direction
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }

    /// Get position along the Ray's direction at the given t-value
    pub fn pos(&self, t: f64) -> Vector {
        return self.origin + self.direction * t;
    }
}

impl ops::Mul<&Ray> for &Matrix {
    type Output = Ray;

    /// Applies matrix transformation to a ray
    fn mul(self, rhs: &Ray) -> Self::Output {
        return Ray::new(self * rhs.origin, self * rhs.direction);
    }
}

/// Data strucutre that represents Intersection (I) of a ray and object
/// t: t-value of Intersection
/// obj: reference to the Drawable Shape
#[derive(Debug, Clone)]
pub struct I {
    pub t: f64,
    pub obj: Rc<RefCell<dyn Drawable>>,
}

impl PartialEq for I {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl I {
    /// Creates a new Intersection
    pub fn new(t: f64, obj: Rc<RefCell<dyn Drawable>>) -> Self {
        Self { t, obj }
    }
}

/// Type that stores a vector of Intersections
pub type Is = Vec<I>;

/// T-Values
pub type Tvalues = Vec<f64>;

#[macro_export]
/// Convenient way to create Tvalues from multiple elements (like vec![...])
macro_rules! tvalues {
    () => {
        vec![]
    };

    ($($el:expr),*) => {{
        let mut res = Vec::new();
        $(
            res.push($el);
        )*
        res
    }};
}

/// Interface for working with the Intersections type
/// II: Intersections (Is) Interface
pub trait II {
    /// Mutably sorts Intersections object
    fn sort(&mut self);

    /// Checks whether the Intersections object contain a given value with EPS precision
    fn contains(&self, val: f64) -> bool;

    /// Returns a Hit from Intersections
    fn hit(&self) -> Option<&I>;

    /// Creates a sorted Intersections object from Tvalues, relating them to the given object (Rc<RefCell<Shape>>)
    fn create(ts: Tvalues, obj: Rc<RefCell<dyn Drawable>>) -> Self;

    /// Combines Intersection (I) into one Intersections object, and sorts them
    fn combine(intersections: &[I]) -> Self;
}

impl II for Is {
    fn sort(&mut self) {
        self.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
    }

    fn contains(&self, val: f64) -> bool {
        for i in self.iter() {
            if utils::feq(i.t, val) {
                return true;
            }
        }

        return false;
    }

    fn hit(&self) -> Option<&I> {
        for i in self.iter() {
            if i.t < 0.0 {
                continue;
            }

            return Some(i);
        }

        return None;
    }

    fn create(ts: Tvalues, obj: Rc<RefCell<dyn Drawable>>) -> Self {
        let mut res = Is::new();
        for t in ts {
            res.push(I::new(t, obj.clone()));
        }
        res.sort();
        res
    }

    fn combine(intersections: &[I]) -> Self {
        let mut res = intersections.to_vec();
        res.sort();
        return res as Is;
    }
}

/// A trait that implements methods used to draw Shape on a Canvas.
pub trait Drawable: Debug {
    /// Draw the object to the Canvas
    /// cv: mutable reference to the Canvas object
    fn draw(&self, _cv: &mut Canvas) {
        panic!("This Drawable object has no implemented draw()");
    }

    /// Explicitely set the transformation to the Drawable object (Shape)
    /// t: owned Transformation object
    fn set_transform(&mut self, _t: Transformation) {
        panic!("This Drawable object has no implemented set_transform()");
    }

    /// Explicitely set a singlular transformation (TUnit) to a Drawable object (Shape)
    /// t_unit: owned TUnit object
    fn set_tunit(&mut self, _t_unit: TUnit) {
        panic!("This Drawable object has no implemented set_tunit()");
    }

    /// Explicitely set the material to the Drawable object (Shape)
    /// m: owned Material object
    fn set_material(&mut self, _m: Material) {
        panic!("This Drawable object has no implemented set_material()");
    }

    /// Returns a reference to the Transformation object of the Drawable object (Shape)
    fn get_transform(&self) -> &Transformation {
        panic!("This Drawable object has no implemented get_transform()");
    }

    /// Returns a reference to the Material object of the Drawable object (Shape)
    fn get_material(&self) -> &Material {
        panic!("This Drawable object has no implemented get_material()");
    }

    /// Returns a mutable reference to the Material object of the Drawable object (Shape)
    fn get_material_mut(&mut self) -> &mut Material {
        panic!("This Drawable object has no implemented get_material_mut()");
    }

    /// Returns a normal vector at a given point on the Drawable object (Shape)
    /// world_p: reference to a world radius-vector of the point (Vector)
    fn normal(&self, world_p: &Vector) -> Vector {
        // inverse transformation matrix
        let itm = self
            .get_transform()
            .inverse()
            .expect("Could not invert Transformation matrix in Shape::Drawable");

        // transform point from World Space to Object Space
        let obj_p = itm * world_p;

        // get local normal
        let obj_n = self.local_normal(&obj_p);

        // transform normal from Object Space to World Space
        let mut world_n = itm.transpose() * obj_n;
        world_n.w = 0.0;

        return world_n.normalize();
    }

    /// Returns a local normal vector at a given point on the Drawable object (Shape)
    /// obj_p: reference to an object radius-vector of the point (Vector)
    fn local_normal(&self, _obj_p: &Vector) -> Vector {
        panic!("This Drawable object has no implemented local_normal()");
    }

    /// (World Space) Returns a SORTED vector of t-values where a given Ray intersects the Drawable object (Shape)
    /// world_r: reference to a world-coordinates Ray which Is are seeked (&Ray)
    fn intersect(&self, world_r: &Ray) -> Tvalues {
        // inverse transformation matrix
        let itm = self
            .get_transform()
            .inverse()
            .expect("Could not invert Transformation matrix in Shape::Drawable");

        // transform ray from World Space to Object Space
        let obj_r = &itm * world_r;
        return self.local_intersect(&obj_r);
    }

    /// (Object Space) Returns a SORTED vector of t-values where a given Ray intersects the Drawable object (Shape)
    /// obj_r: reference to an object_coordinates Ray which Is are seeked (&Ray)
    fn local_intersect(&self, _obj_r: &Ray) -> Tvalues {
        panic!("This Drawable object has no implemented local_intersect()");
    }

    /// Wraps Drawable object into Rc<RefCell<dyn Drawable>>
    fn wrap(self) -> Rc<RefCell<dyn Drawable>>
    where
        Self: Sized + 'static,
    {
        Rc::new(RefCell::new(self))
    }

    /// Wraps Drawable object into Box<dyn Drawable>
    fn wrap_box(self) -> Box<dyn Drawable>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

/// An abstract data structure that represents a shape drawable onto a Canvas
#[derive(Debug, Clone, Default)]
pub struct Shape {
    /// Transformation object
    pub t: Transformation,

    /// Material of a Shape
    pub m: Material,
}

impl Drawable for Shape {
    fn get_material(&self) -> &Material {
        return &self.m;
    }

    fn set_transform(&mut self, t: Transformation) {
        self.t = t;
    }

    fn set_tunit(&mut self, t_unit: TUnit) {
        self.t = transform!(t_unit);
    }

    fn set_material(&mut self, m: Material) {
        self.m = m;
    }

    fn get_transform(&self) -> &Transformation {
        return &self.t;
    }

    fn get_material_mut(&mut self) -> &mut Material {
        return &mut self.m;
    }
}

/// Point Light Source
/// pos: world-coordinates position of the point light source
/// int: intensity of the light source (measured in [Color])
pub struct PointLight {
    pos: Vector,
    int: Color,
}

impl PointLight {
    /// Creates a new PointLight
    pub fn new(pos: Vector, int: Color) -> Self {
        Self { pos, int }
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

impl PointLight {
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
