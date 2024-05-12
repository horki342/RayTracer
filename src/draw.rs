use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::math::linalg::MatrixMethods;

use super::math::color::Color;
use super::math::linalg::Matrix;

/// Structure that implements Canvas where objects are drawn (which is just a matrix with special methods)
pub type Canvas = Matrix<Color>;

impl Canvas {
    /// returns a blank canvas
    pub fn blank(width: usize, height: usize) -> Canvas {
        Canvas::new_with_value(width, height, Color::black())
    }

    pub fn to_ppm(&self, dir: &str, filename: &str) {
        // insert PPM flavor, width, and height
        let mut ppm = String::new();
        ppm.push_str(format!("P3\n{} {}\n255\n", self.width, self.height).as_str());

        // write pixels to ppm
        let mut buf = String::new();
        for i in 0..self.height {
            buf.clear();
            for j in 0..self.width {
                buf.push_str(self.grid[i * self.width + j].fmt().as_str());
                buf.push(' ');
            }
            ppm.push_str(buf.trim());
            ppm.push('\n');
        }
        ppm = ppm.trim().to_owned();

        // load ppm string into a file on a given path (dir/filename)
        self.ppm_to_file(dir, filename, ppm.as_bytes());
    }

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
