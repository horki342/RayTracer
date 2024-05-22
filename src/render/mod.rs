use self::shapes::Drawable;

use super::math::Color;

use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::ops;
use std::path::PathBuf;
use std::rc::Rc;

pub mod core;
pub mod shapes;

/// Structure that is used to generate images on Canvas and PPM, configure the World and Camera
pub struct Renderer {
    pub world: World,
    cv: Canvas,
}

impl Renderer {
    /// Creates a new empty world, and canvas with specified dimensions
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            world: World::new(),
            cv: Canvas::new(width, height),
        }
    }

    /// Render objects from the world onto the canvas
    pub fn render(&mut self) {
        for obj in &self.world.objects {
            obj.borrow_mut().draw(&mut self.cv);
        }
    }

    /// Generates the PPM file
    pub fn generate_ppm(&self, filename: &str) {
        self.cv.to_ppm(filename);
    }

    /// Resets background of the canvas
    pub fn reset(&mut self, bg: Color) {
        self.cv.reset(bg);
    }
}

/// Structure that holds objects, their inner data, and overall configurations of the virtual world
pub struct World {
    pub objects: Vec<Rc<RefCell<dyn Drawable>>>,
}

impl World {
    /// Creates an empty World
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    /// Adds an object
    pub fn add(&mut self, obj: Rc<RefCell<dyn Drawable>>) {
        self.objects.push(obj);
    }
}

/// Implements Canvas where objects are drawn. Canvas can be converted to PPM format to be visualized.
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    grid: Vec<Color>,
}

impl Canvas {
    /// Creates an instance of Canvas
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
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

    /// Resets Canvas with a provided background color.
    pub fn reset(&mut self, bg: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.write(x, y, bg) {
                    Ok(_) => (),
                    Err(err_text) => eprintln!(
                        "Error occured: {}\nSo filling with the default color",
                        err_text
                    ),
                }
            }
        }
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
