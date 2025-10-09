use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Circle {
    pub radius: f64,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Square {
    pub side_length: f64,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Rectangle {
    pub length: f64,
    pub width: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    pub fn perimeter(&self) -> f64 {
        std::f64::consts::PI * self.radius * 2.0
    }
    pub fn scale(&mut self, scale: f64) {
        self.radius *= scale;
    }

}
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A Circle with radius {}", self.radius)
    }
}

impl Square {
    pub fn area(&self) -> f64 {
        self.side_length.powf(2.0)
    }
    pub fn perimeter(&self) -> f64 {
        self.side_length * 4.0
    }
    pub fn scale(&mut self, scale: f64) {
        self.side_length *= scale;
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A Square with side length {}", self.side_length)
    }
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.length* self.width
    }
    pub fn perimeter(&self) -> f64 {
        (self.length * 2.0) + (self.width * 2.0)
    }
    pub fn scale(&mut self, scale: f64) {
        self.length *= scale;
        self.width *= scale;
    }
}
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A Rectangle with length {} and width {}", self.length, self.width)
    }
}