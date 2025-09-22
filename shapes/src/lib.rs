mod shapes;

#[cfg(test)]
mod tests {
    use crate::shapes::{Circle, Rectangle, Square};

    #[test]
    fn can_create_circle() {
        let c = Circle { radius: 3.14 };
        assert_eq!(c.radius, 3.14);
    }
    #[test]
    fn can_create_square() {
        let s = Square { side_length: 4 };
        assert_eq!(s.side_length, 4);
    }
    #[test]
    fn can_create_rectangle() {
        let r = Rectangle { length: 5, width: 6 };
        assert_eq!(r.length, 5);
        assert_eq!(r.width, 6);

    }
    #[test]
    fn can_calculate_circle_area() {
        let c = Circle { radius: 3.14 };
        let area = c.area();
        assert_eq!(area, std::f64::consts::PI * 3.14 * 3.14);
    }
    #[test]
    fn can_calculate_square_area() {
        let s = Square { side_length: 5};
        let area = s.area();
        assert_eq!(area, 5.0 * 5.0);

    }
    #[test]
    fn can_calculate_rectangle_area() {
        let r = Rectangle { length: 5, width: 6 };
        let area = r.area();
        assert_eq!(area, 6.0 * 5.0);

    }
    #[test]
    fn can_calculate_circle_perimeter() {
        let c = Circle { radius: 3.14 };
        let perimeter = c.perimeter();
        assert_eq!(perimeter, std::f64::consts::PI * 3.14 * 2.0);

    }
    #[test]
    fn can_calculate_square_perimeter() {
        let s = Square { side_length: 5 };
        let perimeter = s.perimeter();
        assert_eq!(perimeter, 5.0 * 4.0);

    }
    #[test]
    fn can_calculate_rectangle_perimeter() {
        let r = Rectangle { length: 5, width: 6 };
        let perimeter = r.perimeter();
        assert_eq!(perimeter, 6.0 *2.0 + 2.0 * 5.0);

    }
}
