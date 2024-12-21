//! Miscellaneuos useful functions

use crate::lib::*;

pub enum Axes3D {
    X,
    Y,
    Z,
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
