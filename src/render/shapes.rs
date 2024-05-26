use std::cell::RefCell;
use std::rc::Rc;

use super::core::Material;
use super::Canvas;

use crate::math::utils::point;
use crate::math::{utils, Color, TUnit};
use crate::math::{Transformation, Vector};
use crate::transform;

/// Trait that allows objects to be drawn on the Canvas.
/// It requires intersect methods that allows to find intersections of a given ray with the object.
pub trait Drawable: std::fmt::Debug {
    /// Draw the object to the Canvas
    fn draw(&self, cv: &mut Canvas);

    /// Set the transformation to the object
    fn set_transform(&mut self, t: Transformation);

    /// Set single transformation to the object
    fn apply_tunit(&mut self, t: TUnit);

    /// Returns a normal vector at a given point
    fn normal(&self, _p: &Vector) -> Vector {
        panic!("Could not find the normal of the Drawable object");
    }

    /// Set a Material
    fn set_material(&mut self, _material: Material) {
        panic!("Could not set Material to the Drawable object");
    }
}

/// Structure that implements a Drawable
#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    /// center of the sphere
    pub c: Vector,

    /// radius of the sphere
    pub r: f64,

    /// transformation of the sphere
    pub t: Transformation,

    /// material
    pub m: Material,
}

impl Sphere {
    /// Creates an instance of the Sphere with default transformation.
    pub fn new(
        c: Vector,
        r: f64,
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            c,
            r,
            t: Transformation::default(),
            m: Material::new(color, ambient, diffuse, specular, shininess),
        }))
    }

    /// Creates a unit sphere centered in the origin.
    pub fn default() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            c: utils::point(0.0, 0.0, 0.0),
            r: 1.0,
            t: Transformation::default(),
            m: Material::default(),
        }))
    }
}

impl Drawable for Sphere {
    fn draw(&self, _cv: &mut Canvas) {
        todo!()
    }

    fn set_transform(&mut self, t: Transformation) {
        self.t = t;
    }

    /// p_wld = point in world coordinates
    fn normal(&self, p_wld: &Vector) -> Vector {
        // transform point
        let m = self.t.inverse().unwrap();
        let p_obj = &m * p_wld;
        let n_obj = p_obj - point(0.0, 0.0, 0.0);
        let mut n_wld = m.transpose() * n_obj;
        n_wld.w = 0.0;
        n_wld.normalize_mut();

        return n_wld;
    }

    fn apply_tunit(&mut self, t: TUnit) {
        self.t = transform!(t);
    }

    fn set_material(&mut self, material: Material) {
        self.m = material;
    }
}

/// Structure that implements a Drawable Point
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pos: Vector,
    t: Transformation,
    color: Color,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64, col: Color) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            pos: utils::point(x, y, z),
            t: transform!(),
            color: col,
        }))
    }
}

impl Drawable for Point {
    fn draw(&self, cv: &mut Canvas) {
        // transformed point
        let trp = &self.t * self.pos;
        let x = trp.x.round() as usize;
        let y = trp.y.round() as usize;

        cv.write(x, y, self.color)
            .expect("Could not draw the Point");
    }

    fn set_transform(&mut self, t: Transformation) {
        self.t = t;
    }

    fn apply_tunit(&mut self, t: TUnit) {
        self.t = transform!(t);
    }
}
