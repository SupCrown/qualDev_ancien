pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod shapes_enum;
pub use shapes_enum::*;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}



use std::f64::consts::PI;
use crate::shapes_enum::{Circle, Rectangle, Square};

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}


impl Shape for Square {
    fn area(&self) -> f64 {
        self.side_lentgh * self.side_lentgh
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side_lentgh
    }
}


impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        self.width * self.height * 2.0
    }
}


