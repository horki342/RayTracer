use shapes::{Point, Sphere};

use crate::math::{utils, Color, Matrix, TUnit, Vector};

use self::core::Drawable;

use core::{Computations, Is, Material, PointLight, RAIIDrawable, Ray, II};
use std::fs::File;
use std::io::Write;
use std::ops;
use std::path::PathBuf;

pub mod core;
pub mod shapes;

/// Structure that implements Camera
pub struct Camera {
    pub hsize: usize, // in px
    pub vsize: usize, // in px
    pub fov: f64,

    pub px_size: f64, // pixel size in canvas untis
    pub hw: f64,      // half width of CV
    pub hh: f64,      // half height of CV

    /// view transformation matrix
    pub vtm: Matrix,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let (px_size, hw, hh) = Camera::calculate_parameters(hsize as f64, vsize as f64, fov);

        Self {
            hsize,
            vsize,
            fov,
            px_size,
            hw,
            hh,
            vtm: Matrix::identity(),
        }
    }

    /// Returns a Ray from the Camera to the provided pixel position of the Canvas
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x = x as f64;
        let y = y as f64;

        // the offset from the edge of the canvas to the pixel's cente
        let xoffset = (x + 0.5) * self.px_size;
        let yoffset = (y + 0.5) * self.px_size;

        // the untransformed coordinates of the pixel in world space.
        let world_x = self.hw - xoffset;
        let world_y = self.hh - yoffset;

        // calculate the inverse of the view transformation
        let inv_view = self
            .vtm
            .try_inverse()
            .expect("Cannot invert view transformation matrix in Camera.ray_for_pixel()");

        // find the ray's origin and direction, and apply the view transformation
        let pixel = &inv_view * utils::point(world_x, world_y, -1.0);
        let origin = &inv_view * utils::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray { origin, direction }
    }

    /// Calculates pixel size, half_width, and half_height of the Canvas
    fn calculate_parameters(hsize: f64, vsize: f64, fov: f64) -> (f64, f64, f64) {
        // half view
        let hv = (fov / 2.0).tan();

        let hw: f64; // half width
        let hh: f64; // half height

        let aspect_ratio = hsize / vsize;

        if aspect_ratio >= 1.0 {
            hw = hv;
            hh = hv / aspect_ratio;
        } else {
            hw = hv * aspect_ratio;
            hh = hv;
        }

        return (hw * 2.0 / hsize, hw, hh);
    }

    /// Sets a camera's view transformation
    pub fn set_view(&mut self, from: Vector, to: Vector, up: Vector) {
        // normalize up vector
        let up = up.normalize();

        // compute the forward, left, and true_up vectors
        let forward = (to - from).normalize();
        let left = utils::cross(&forward, &up);
        let true_up = utils::cross(&left, &forward);

        // compute orientation matrix
        let orientation = utils::matrix(
            left.x, left.y, left.z, 0.0, true_up.x, true_up.y, true_up.z, 0.0, -forward.x,
            -forward.y, -forward.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        // translate orientation matrix
        let view_matrix = orientation * (TUnit::Translate(-from.x, -from.y, -from.z).matrix());

        self.vtm = view_matrix;
    }
}

/// Structure that is used to generate images on Canvas and PPM, configure the World and Camera
pub struct Renderer {
    pub world: World,
    cv: Canvas,
    c: Camera,
}

impl Renderer {
    /// Creates a new empty world, and canvas with specified dimensions
    pub fn new(
        hsize: usize,
        vsize: usize,
        fov: f64,
        from: Vector,
        to: Vector,
        up: Vector,
        bg: Color,
    ) -> Self {
        let mut res = Self {
            world: World::new(),
            cv: Canvas::new(hsize, vsize, bg),
            c: Camera::new(hsize, vsize, fov),
        };
        res.c.set_view(from, to, up);
        res
    }

    /// Render objects from the world onto the canvas
    pub fn render(&mut self) {
        for y in 0..self.cv.height {
            for x in 0..self.cv.width {
                let ray = self.c.ray_for_pixel(x, y);
                let color = self.world.calc(&ray, &self.cv.bg);
                self.cv
                    .write(x, y, color)
                    .expect("Could not write to Canvas at Renderer.render()");
            }
        }

        // Draw points
        for p in self.world.points.iter() {
            p.draw(&mut self.cv);
        }
    }

    /// Generates the PPM file
    pub fn generate_ppm(&self, filename: &str) {
        self.cv.to_ppm(filename);
    }
}

/// Structure that holds points, objects and lights, their inner data, and overall configurations of the virtual world
pub struct World {
    pub points: Vec<Point>,
    pub objects: Vec<RAIIDrawable>,
    pub sources: Vec<Box<PointLight>>,
}

impl World {
    /// Creates an empty World
    pub fn new() -> Self {
        Self {
            points: vec![],
            objects: vec![],
            sources: vec![],
        }
    }

    /// Adds a point
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    /// Adds an object
    pub fn add_obj(&mut self, obj: RAIIDrawable) {
        self.objects.push(obj);
    }

    /// Adds objects
    pub fn add_objs(&mut self, objs: Vec<RAIIDrawable>) {
        for obj in objs {
            self.add_obj(obj);
        }
    }

    /// Adds a light source
    pub fn add_src(&mut self, src: Box<PointLight>) {
        self.sources.push(src);
    }

    /// Interect the world's object with a given ray
    pub fn intersect(&self, r: &Ray) -> Is {
        let mut world_intersections: Is = Is::new();

        for el in self.objects.iter() {
            // calculate t-values
            let ts = el.borrow().intersect(r);
            let mut xs = Is::create(ts, el.clone());
            world_intersections.append(&mut xs);
        }

        world_intersections.sort();
        world_intersections
    }

    /// Checks whether a point is shadowed
    /// p: point that is being checked
    pub fn is_shadowed(&self, p: &Vector) -> bool {
        // todo!("Support multiple light sources")
        if self.sources.len() != 1 {
            panic!("World does not support multiple sources, or no sources were provided");
        }

        // calculate the distance from the point p to the light source
        let mut v = self.sources[0].pos - p;
        let dist = v.magnitude();

        // get the ray from the point p to the light source
        v.normalize_mut();
        let r = Ray::new(p.clone(), v);

        // intersect world with the ray, and identify hit
        let xs = self.intersect(&r);
        let hit = xs.hit();

        match hit {
            Some(i) => {
                if i.t - dist < -crate::math::utils::EPSILON / 2.0 {
                    return true;
                }
                return false;
            }
            None => return false,
        }
    }

    /// Shades a hit using given computations information
    pub fn shade_hit(&self, info: Computations) -> Color {
        // todo!("Support multiple light sources");
        if self.sources.len() != 1 {
            panic!("World does not support multiple sources, or no sources were provided");
        }

        // determine whether the point is shadowed
        let shadowed: bool = self.is_shadowed(&info.over_p);

        return self.sources[0].shade(
            info.obj.borrow().get_material(),
            &info.p,
            &info.e,
            &info.n,
            shadowed,
        );
    }

    /// Calculate color in the World when the Ray is travelling
    pub fn calc(&self, r: &Ray, bg: &Color) -> Color {
        // todo!("Hit returns &I, so for performance purposes it can take the ownership, so that clone is not necessary.")
        let xs = self.intersect(&r);
        let hit = xs.hit();

        match hit {
            Some(i) => {
                return self.shade_hit(Computations::new(i.clone(), r));
            }
            None => bg.clone(), // ray hit nothing.
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let mut s1 = Sphere::default();
        let s1_m: &mut Material = s1.get_material_mut();
        s1_m.color = utils::color(0.8, 1.0, 0.6);
        s1_m.diffuse = 0.7;
        s1_m.specular = 0.2;

        let mut s2 = Sphere::default();
        s2.set_tunit(TUnit::Scale(0.5, 0.5, 0.5));

        let light = PointLight::new(
            utils::point(-10.0, 10.0, -10.0),
            utils::color(1.0, 1.0, 1.0),
        );

        let mut world = World::new();
        world.add_obj(s1.wrap());
        world.add_obj(s2.wrap());
        world.add_src(light.wrap_box());

        world
    }
}
 
/// Implements Canvas where objects are drawn. Canvas can be converted to PPM format to be visualized.
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub bg: Color,
    grid: Vec<Color>,
}

impl Canvas {
    /// Creates an instance of Canvas
    pub fn new(width: usize, height: usize, bg: Color) -> Canvas {
        Canvas {
            width,
            height,
            bg,
            grid: vec![Color::default(); width * height],
        }
    }

    /// Writes a color to the pixel at a given position
    pub fn write(&mut self, x: usize, y: usize, val: Color) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            return Err("Canvas.write(): pixel out of bounds");
        }

        self[[x, y]] = val;
        Ok(())
    }

    /// Converts (x, y) coordinates to (z) coordinates (matrix to grid)
    fn cc(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    /// Converts Canvas to ppm format and writes it to the ppm file in img directory.
    pub fn to_ppm(&self, filename: &str) {
        // insert PPM flavor, width, and height
        let mut ppm = String::new();
        ppm.push_str(format!("P3\n{} {}\n255\n", self.width, self.height).as_str());

        // write pixels to ppm
        let mut buf = String::new();
        for i in 0..self.height {
            buf.clear();
            for j in 0..self.width {
                buf.push_str(self[[j, i]].fmt().as_str());
                buf.push(' ');
            }
            ppm.push_str(buf.trim());
            ppm.push('\n');
        }
        ppm = ppm.trim().to_owned();

        // load ppm string into a file on a given path (dir/filename)
        // dir must be 'img'
        // filename should have .ppm extension to be displayed correctly
        match self.ppm_to_file("img", filename, ppm.as_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }

    /// Writes buffer (PPM-format) to the dir/filename.ppm
    fn ppm_to_file(&self, dir: &str, filename: &str, buf: &[u8]) -> Result<(), &'static str> {
        // open file to read
        let path = match PathBuf::from(dir).join(filename).to_str() {
            Some(res) => String::from(res),
            None => return Err("Canvas.ppm_to_file(): Unable to convert PathBuf to &str"),
        };

        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(_) => return Err("Canvas.ppm_to_file(): Could not open the file"),
        };

        match file.write(buf) {
            Ok(_) => return Ok(()),
            Err(_) => return Err("Canvas.ppm_to_file(): Could not write to the file"),
        }
    }
}

impl ops::Index<[usize; 2]> for Canvas {
    type Output = Color;

    /// row, col
    fn index(&self, ind: [usize; 2]) -> &Self::Output {
        let idx = self.cc(ind[0], ind[1]);
        &self.grid[idx]
    }
}

impl ops::IndexMut<[usize; 2]> for Canvas {
    fn index_mut(&mut self, ind: [usize; 2]) -> &mut Self::Output {
        let idx = self.cc(ind[0], ind[1]);
        &mut self.grid[idx]
    }
}
