use std::ops;

/// structure to hold Colors
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

impl Color {
    // implement default colors
    pub fn black() -> Self {
        Color::default()
    }

    /// converts val from 0..1 to 0..255 (for colors)
    fn cvt(val: f32) -> i32 {
        if val > 1.0 {
            return 255;
        }
        if val < 0.0 {
            return 0;
        }

        (val * 255.0).round() as i32
    }

    pub fn fmt(&self) -> String {
        format!(
            "{} {} {}",
            Color::cvt(self.red),
            Color::cvt(self.green),
            Color::cvt(self.blue)
        )
    }
}

/// returns a color with provided RGB values
#[macro_export]
macro_rules! color {
    ($red:expr, $green:expr, $blue:expr) => {
        crate::math::color::Color {
            red: $red,
            green: $green,
            blue: $blue,
        }
    };
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        return crate::math::equal(self.red, other.red)
            && crate::math::equal(self.green, other.green)
            && crate::math::equal(self.blue, other.blue);
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl ops::Add<Color> for Color {
    type Output = Self;

    /// adds two colors
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl ops::Sub<Color> for Color {
    type Output = Self;

    /// substract two colors
    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;

    /// multiplies a color with a number
    fn mul(self, rhs: f32) -> Self::Output {
        color![self.red * rhs, self.green * rhs, self.blue * rhs]
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    /// multiplies a number with a tuple
    fn mul(self, rhs: Color) -> Self::Output {
        color![self * rhs.red, self * rhs.green, self * rhs.blue]
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    /// defines Schur multiplication for colors
    fn mul(self, rhs: Color) -> Self::Output {
        color![
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue
        ]
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;

    /// divides a color with a number
    fn div(self, rhs: f32) -> Self::Output {
        if crate::math::equal(0.0, rhs) {
            panic!("Zero-division error (Color / f32)");
        }

        // faster than four different divisions
        return self * (1.0 / rhs);
    }
}
