use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::math::color::Color;

/// Structure that implements Canvas where objects are drawn
#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Color>>,
}

impl Canvas {
    /// create a new black canvas
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width: width,
            height: height,
            grid: vec![vec![Color::black(); width]; height],
        }
    }

    /// writes color to a pixel in a given location
    pub fn write(&mut self, col: usize, row: usize, color: Color) {
        self.grid[row][col] = color;
    }

    /// return a color of a pixel in a given location
    pub fn at(&self, col: usize, row: usize) -> &Color {
        &self.grid[row][col]
    }

    pub fn to_ppm(&self, dir: &str, filename: &str) {
        // insert PPM flavor, width, and height
        let mut ppm = String::new();
        ppm.push_str(format!("P3\n{} {}\n255\n", self.width, self.height).as_str());

        // write pixels to ppm
        let mut buf = String::new();
        for row in &self.grid {
            buf.clear();
            for pixel in row {
                buf.push_str(pixel.fmt().as_str());
                buf.push(' ');
            }
            ppm.push_str(buf.trim());
            ppm.push('\n');
        }
        ppm = ppm.trim().to_owned();

        // load ppm string into a file on a given path (dir/filename)
        self.ppm_to_file(dir, filename, ppm.as_bytes());
    }

    #[allow(dead_code)]
    fn ppm_to_file(&self, dir: &str, filename: &str, buf: &[u8]) {
        // open file to read
        let path = match PathBuf::from(dir).join(filename).to_str() {
            Some(res) => String::from(res),
            None => panic!("Unable to convert PathBuf to &str"),
        };

        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(_) => panic!("Could not open the file"),
        };

        file.write(buf).expect("Could not write to the file");
    }
}
