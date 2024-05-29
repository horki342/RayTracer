//! Contains all Drawable Shapes and other Drawable Objects
//! Each specific shape has a "shape" field that contains general
//! functionality of each Drawable object

use super::core::*;

use crate::math::{utils, Vector};
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
    fn set_transform(&mut self, t: math::Transformation) {
        self.shape.set_transform(t)
    }

    fn set_tunit(&mut self, t_unit: math::TUnit) {
        self.shape.set_tunit(t_unit)
    }

    fn set_material(&mut self, m: Material) {
        self.shape.set_material(m)
    }

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

    fn get_transform(&self) -> &math::Transformation {
        self.shape.get_transform()
    }

    fn get_material(&self) -> &Material {
        self.shape.get_material()
    }

    fn get_material_mut(&mut self) -> &mut Material {
        self.shape.get_material_mut()
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
    fn set_transform(&mut self, t: math::Transformation) {
        self.shape.set_transform(t);
    }

    fn set_tunit(&mut self, t_unit: math::TUnit) {
        self.shape.set_tunit(t_unit);
    }

    fn set_material(&mut self, m: Material) {
        self.shape.set_material(m);
    }

    fn get_transform(&self) -> &math::Transformation {
        self.shape.get_transform()
    }

    fn get_material(&self) -> &Material {
        self.shape.get_material()
    }

    fn get_material_mut(&mut self) -> &mut Material {
        self.shape.get_material_mut()
    }
}

// end Point ===========================================================================================
