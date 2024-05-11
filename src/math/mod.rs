//! This modules provides a math framework that is fast and easy to work with.

pub mod color;
pub mod linalg;

/// constants
const EPS: f32 = 1e-5;

/// compares two float numbers
fn equal(a: f32, b: f32) -> bool {
    if (a - b).abs() < EPS {
        return true;
    }
    return false;
}
