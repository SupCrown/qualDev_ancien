pub struct Circle {
    pub radius: f64,
}
pub struct Square {
    pub side_length: i32,
}

pub struct Rectangle {
    pub length: i32,
    pub width: i32,
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    pub fn perimeter(&self) -> f64 {
        std::f64::consts::PI * self.radius * 2.0
    }
}

impl Square {
    pub fn area(&self) -> f64 {
        self.side_length.pow(2) as f64
    }
    pub fn perimeter(&self) -> f64 {
        (self.side_length * 4) as f64
    }
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.length as f64 * self.width as f64
    }
    pub fn perimeter(&self) -> f64 {
        ((self.length  * 2) + (self.width * 2)) as f64
    }
}