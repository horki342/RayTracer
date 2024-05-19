use crate::draw::*;
use crate::intersections;
use std::borrow::Borrow;
use std::rc::Rc;

pub trait Drawable {
    fn draw(&self, cv: &mut Canvas);
    fn transform(&mut self, pipeline: Pipeline);
    fn get_origin(&self) -> Vector;

    fn get_info(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    color: Color,
}

impl Material {
    pub fn new(color: Color) -> Self {
        Material { color: color }
    }

    pub fn change_color(&mut self, new_color: Color) {
        self.color = new_color;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    origin: Vector,
    direction: Vector,
    material: Material,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector, color: Color) -> Self {
        Ray {
            origin,
            direction,
            material: Material::new(color),
        }
    }

    pub fn pos(&self, t: f64) -> Vector {
        return self.origin + self.direction * t;
    }

    pub fn intersect_sphere(&self, sphere: Rc<Sphere>) -> Intersections<Sphere> {
        let del = self.origin - sphere.center;

        let a = dot(&del, &del) - sphere.radius;
        let b = dot(&self.direction, &del);
        let c = dot(&self.direction, &self.direction);

        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return intersections!();
        } else {
            let t1 = (-b + discriminant.sqrt()) / c;
            let t2 = (-b - discriminant.sqrt()) / c;

            return intersections!(t1, sphere; t2, sphere.clone(););
        }
    }
}

impl Drawable for Ray {
    fn draw(&self, cv: &mut Canvas) {
        let mut t = 0.0;
        loop {
            let p = self.pos(t);
            let x = p.x as usize;
            let y = p.y as usize;

            match cv.write(x, y, self.material.color) {
                Ok(_) => t += 0.001,
                Err(err_text) => {
                    println!("{}", err_text);
                    break;
                }
            }
        }
    }

    fn get_origin(&self) -> Vector {
        return self.origin;
    }

    fn transform(&mut self, pipeline: Pipeline) {
        let matrix = pipeline.get_matrix();

        self.origin = matrix * self.origin;
        self.direction = matrix * self.direction;
    }

    fn get_info(&self) -> String {
        format!("{:#?}", self)
    }
}

#[derive(Clone)]
pub struct Intersection<T: Drawable> {
    t: f64,
    obj: Rc<T>,
}

impl<T: Drawable + PartialEq> PartialEq for Intersection<T> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.obj == other.obj
    }
}

impl<T: Drawable> Intersection<T> {
    pub fn new(t: f64, obj: Rc<T>) -> Self {
        Self { t, obj }
    }

    pub fn has(&self, t: f64) -> bool {
        return t == self.t;
    }
}

impl<T: Drawable> std::fmt::Debug for Intersection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Intersection")
            .field("t", &self.t)
            .field("obj", &self.obj.get_info())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Intersections<T: Drawable> {
    data: Vec<Intersection<T>>,
}

impl<T: Drawable> Intersections<T> {
    pub fn from(data: Vec<Intersection<T>>) -> Self {
        Self { data }
    }

    pub fn has(&self, t: f64) -> bool {
        for el in &self.data {
            if el.has(t) {
                return true;
            }
        }

        return false;
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn hit(&self) -> Option<&Intersection<T>> {
        let res = self
            .data
            .iter()
            .min_by(|a, b| a.t.abs().partial_cmp(&(b.t.abs())).unwrap());
        match res {
            Some(val) => {
                if val.t < 0.0 {
                    return None;
                }
                return Some(val);
            }
            None => return None,
        }
    }
}

#[macro_export]
macro_rules! intersections {
    () => {
        Intersections::from(Vec::new())
    };

    ($($i:expr),* $(,)?) => {{
        use crate::draw::shapes::Intersections;

        let mut vec = Vec::new();
        $(
            vec.push($i);
        )*
        Intersections::from(vec)
    }};

    ($t:expr, $obj:expr; $($rest:tt)*) => {{
        let mut vec = intersections!($($rest)*).data;
        vec.push(Intersection::new($t, $obj));
        Intersections::from(vec)
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    point: Vector,
    material: Material,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64, color: Color) -> Self {
        Point {
            point: point(x, y, z),
            material: Material::new(color),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, cv: &mut Canvas) {
        let x = self.point.x.round() as usize;
        let y = self.point.y.round() as usize;

        match cv.write(x, y, self.material.color) {
            Ok(_) => (),
            Err(err_text) => {
                eprintln!("{}", err_text);
            }
        }
    }

    fn get_origin(&self) -> Vector {
        return self.point;
    }

    fn transform(&mut self, pipeline: Pipeline) {
        self.point = pipeline.get_matrix() * self.point;
    }

    fn get_info(&self) -> String {
        format!("{:#?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, r: f64, color: Color) -> Self {
        Sphere {
            center: point(x, y, z),
            radius: r,
            material: Material::new(color),
        }
    }
}

impl Drawable for Sphere {
    fn draw(&self, cv: &mut Canvas) {
        todo!()
    }

    fn transform(&mut self, pipeline: Pipeline) {
        todo!()
    }

    fn get_origin(&self) -> Vector {
        todo!()
    }

    fn get_info(&self) -> String {
        format!("{:#?}", self)
    }
}
