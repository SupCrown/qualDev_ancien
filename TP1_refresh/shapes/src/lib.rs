pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod shapes_enum;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

