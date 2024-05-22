use std::cell::RefCell;
use std::rc::Rc;

use super::Canvas;

use crate::math::{utils, Color};
use crate::math::{Transformation, Vector};
use crate::transform;

/// Trait that allows objects to be drawn on the Canvas.
/// It requires intersect methods that allows to find intersections of a given ray with the object.
pub trait Drawable: std::fmt::Debug {
    /// Draw the object to the Canvas
    fn draw(&self, cv: &mut Canvas);

    /// Set the transformation to the object
    fn set_transform(&mut self, t: Transformation);
}

/// Structure that implements a Drawable Sphere
#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    /// center of the sphere
    pub c: Vector,

    /// radius of the sphere
    pub r: f64,

    /// transformation of the sphere
    pub t: Transformation,
}

impl Sphere {
    /// Creates an instance of the Sphere with default transformation.
    pub fn new(c: Vector, r: f64) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            c,
            r,
            t: Transformation::default(),
        }))
    }

    /// Creates a unit sphere centered in the origin.
    pub fn default() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            c: utils::point(0.0, 0.0, 0.0),
            r: 1.0,
            t: Transformation::default(),
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
}
