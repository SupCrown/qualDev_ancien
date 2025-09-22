pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Circle {
    radius : f64,
}

struct Square {
    side_lentgh : f64,
}

struct Rectangle {
    width : f64,
    height: f64,
}

impl Circle {
    fn circle_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    fn circle_perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Square {
    fn square_area(&self) -> f64 {
        self.side_lentgh * self.side_lentgh
    }

    fn square_perimeter(&self) -> f64 {
        4.0 * self.side_lentgh
    }
}

impl Rectangle {
    fn rectangle_area(&self) -> f64 {
        self.width * self.height
    }
    fn rectangle_perimeter(&self) -> f64 {
        self.width * self.height * 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_circle(){
        let a_circle = Circle {radius: 3.14};
        assert_eq!(a_circle.radius, 3.14);
    }

    #[test]
    fn can_create_square(){
        let square = Square {side_lentgh: 3.14};
        assert_eq!(square.side_lentgh, 3.14);
    }

    #[test]
    fn can_create_rectangle(){
        let a_rectangle = Rectangle {width: 3.14, height: 3.14};
        assert_eq!(a_rectangle.width, 3.14);
    }

    #[test]
    fn can_calculate_area_and_perimeter_circle(){
        let circle = Circle {radius: 3.14};
        assert_eq!(circle.circle_area(), std::f64::consts::PI * circle.radius.powi(2));
        assert_eq!(circle.circle_perimeter(), 2.0 * std::f64::consts::PI * circle.radius.powi(2));
    }

    #[test]
    fn can_calculate_area_and_perimeter_square(){
        let square = Square {side_lentgh: 3.14};
        assert_eq!(square.square_area(), square.side_lentgh * square.side_lentgh);
    }
}
