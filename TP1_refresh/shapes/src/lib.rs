pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod shapes_enum;

pub trait Shapetrait {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}



use std::f64::consts::PI;
use crate::shapes_enum::{Circle, Rectangle, Square};

impl Shapetrait for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}


impl Shapetrait for Square {
    fn area(&self) -> f64 {
        self.side_lentgh * self.side_lentgh
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side_lentgh
    }
}


impl Shapetrait for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        self.width * self.height * 2.0
    }
}

pub fn total_area(shapes: &[&dyn Shapetrait]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_area_mixed_from_lib() {
        let c = Circle { radius: 1.0 };
        let s = Square { side_lentgh: 2.0 };
        let r = Rectangle { width: 3.0, height: 4.0 };

        let shapes: Vec<&dyn Shapetrait> = vec![&c, &s, &r];
        let expected = c.area() + s.area() + r.area();
        assert!((total_area(&shapes) - expected).abs() < 1e-12);
    }
}




