use crate::draw::*;

pub trait Drawable {
    fn draw(&self, cv: &mut Canvas);
    fn transform(&mut self, pipeline: Pipeline);
    fn get_origin(&self) -> Vector;
}

#[derive(Debug)]
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

#[derive(Debug)]
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

    pub fn intersect_sphere<'a>(&self, sphere: &'a Sphere) -> Intersections<'a> {
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

            return intersections!(Intersection::new(t1, sphere), Intersection::new(t2, sphere));
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
}

#[derive(Debug)]
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
}

#[derive(Debug)]
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
}
