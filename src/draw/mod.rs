use super::math::*;
use shapes::*;
use std::rc::Rc;
use std::{fs::File, io::Write as _, ops, path::PathBuf};

pub mod shapes;

pub struct Scene {
    cv: Canvas,
    objs: Vec<Box<dyn Drawable>>,
}

impl Scene {
    pub fn new(width: usize, height: usize) -> Self {
        Scene {
            cv: Canvas::new(width, height),
            objs: Vec::new(),
        }
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.cv.reset(bg);
    }

    pub fn add(&mut self, obj: Box<dyn Drawable>) {
        self.objs.push(obj);
    }

    pub fn reset(&mut self, bg: Color) {
        self.objs.clear();
        self.cv.reset(bg)
    }

    pub fn draw(&mut self, name: &str) {
        for obj in &self.objs {
            obj.draw(&mut self.cv);
        }

        self.cv.to_ppm(name);
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            grid: vec![Color::default(); width * height],
        }
    }

    pub fn write(&mut self, x: usize, y: usize, val: Color) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            return Err("Canvas.write(): pixel out of bounds");
        }

        self[[x, y]] = val;
        Ok(())
    }

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

    pub fn _at(&self, x: usize, y: usize) -> Result<Color, &'static str> {
        if x >= self.width || y >= self.height {
            return Err("Canvas.write(): pixel out of bounds");
        }

        return Ok(self[[x, y]]);
    }

    /// convert (x, y) coordinates to (z) coordinates (matrix to grid)
    fn cc(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

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
        match self.ppm_to_file("img", filename, ppm.as_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }

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
