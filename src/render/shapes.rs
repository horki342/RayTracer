//! Contains all Drawable Shapes and other Drawable Objects
//! Each specific shape has a "shape" field that contains general
//! functionality of each Drawable object

use super::core::*;

use crate::math::{utils, Vector, utils::EPSILON};
use crate::{math, tvalues};

// begin Sphere ===========================================================================================

#[derive(Debug, Clone)]
pub struct Sphere {
    shape: Shape,

    /// Center of the sphere
    pub c: Vector,

    /// Radius of the sphere
    pub r: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            shape: Shape::default(),
            c: utils::point(0.0, 0.0, 0.0),
            r: 1.0,
        }
    }
}

impl Drawable for Sphere {
    fn local_normal(&self, obj_p: &Vector) -> Vector {
        let obj_n = obj_p - self.c;
        return obj_n;
    }

    fn local_intersect(&self, obj_r: &Ray) -> Tvalues {
        let del = obj_r.origin - self.c;

        let a = utils::dot(&del, &del) - self.r;
        let b = utils::dot(&obj_r.direction, &del);
        let c = utils::dot(&obj_r.direction, &obj_r.direction);

        // Calculate Discriminant
        let d = b * b - a * c;

        // if there are intersections, return their t-values
        if d >= 0.0 {
            let t1 = (-b + d.sqrt()) / c;
            let t2 = (-b - d.sqrt()) / c;

            return tvalues!(t1, t2);
        }

        // otherwise return an empty vector
        tvalues!()
    }
    
    fn get_shape(&self) -> &Shape {
        &self.shape
    }

    fn get_shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }
}

// end Sphere ===========================================================================================

// begin Point ===========================================================================================

#[derive(Debug, Clone)]
pub struct Point {
    shape: Shape,

    /// Position of the point
    pos: Vector,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64, col: math::Color) -> Self {
        let mut res = Self {
            shape: Shape::default(),
            pos: utils::point(x, y, z),
        };

        res.get_material_mut().change_color(col);
        res
    }

    /// Draws a point to the Canvas
    pub fn draw(&self, cv: &mut super::Canvas) {
        // transformed point
        let trp = self.get_transform() * self.pos;
        let x = trp.x.round() as usize;
        let y = trp.y.round() as usize;

        cv.write(x, y, self.get_material().color)
            .expect("Could not draw the Point");
    }
}

impl Drawable for Point {
    fn get_shape(&self) -> &Shape {
        &self.shape
    }

    fn get_shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }

    fn local_normal(&self, _obj_p: &Vector) -> Vector {
        panic!("Point does not imlepement local_normal")
    }

    fn local_intersect(&self, _obj_r: &Ray) -> Tvalues {
        panic!("Point does not implement local_intersect")
    }
}

// end Point ===========================================================================================

// begin Plane ===========================================================================================

/// Plane that (by default) extends in x- and z-directions.
#[derive(Debug, Clone)]
pub struct Plane {
    shape: Shape,
}

impl Default for Plane {
    fn default() -> Self {
        Plane {
            shape: Shape::default()
        }
    }
}

impl Drawable for Plane {
    fn get_shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }

    fn get_shape(&self) -> &Shape {
        &self.shape
    }

    /// Constant normal for a plane
    fn local_normal(&self, _obj_p: &Vector) -> Vector {
        utils::vector(0.0, 1.0, 0.0)
    }

    fn local_intersect(&self, obj_r: &Ray) -> Tvalues {
        // if ray is parallel to plane, no intersect 
        if obj_r.direction.y.abs() < EPSILON {
            return Tvalues::new();
        }

        let t = -obj_r.origin.y / obj_r.direction.y;
        return vec![t] as Tvalues;
    }
 }

// end Plane ===========================================================================================
