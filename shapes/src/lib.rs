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
        assert_eq!(area, 30.974846927333925);
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

    #[test]
    fn can_rescale_rectangle() {
        let r = Rectangle { length: 5, width: 6 };
        let r2 = Rectangle { length: 10, width: 12 };
        r.scale(2) ;
        assert_eq!(r, r2);

    }
    #[test]
    fn can_rescale_square() {
        let s = Square { side_length: 5 };
        let s2 = Square { side_length: 10};
        s.scale(2) ;
        assert_eq!(s, s2);

    }
    #[test]
    fn can_rescale_circle() {
        let c = Circle { radius: 3.14 };
        let c2 = Circle { radius: 3.14*10.0};
        s.scale(2) ;
        assert_eq!(c, c2);
    }
    #[test]
    fn can_rescale_zero_rectangle() {
        let r = Rectangle { length: 5, width: 6 };
        let r2 = Rectangle { length: 5*0, width: 6*0 };
        r.scale(0) ;
        assert_eq!(r, r2);

    }
    #[test]
    fn can_rescale_zero_square() {
        let s = Square { side_length: 5 };
        let s2 = Square { side_length: 5*0};
        s.scale(0) ;
        assert_eq!(s, s2);

    }
    #[test]
    fn can_rescale_zero_circle() {
        let c = Circle { radius: 3.14 };
        let c2 = Circle { radius: 3.14*0.0};
        s.scale(0) ;
        assert_eq!(c, c2);
    }
    #[test]
    fn can_rescale_big_rectangle() {
        let r = Rectangle { length: 5, width: 6 };
        let r2 = Rectangle { length: 5*100000, width: 6*100000 };
        r.scale(100000) ;
        assert_eq!(r, r2);

    }
    #[test]
    fn can_rescale_big_square() {
        let s = Square { side_length: 5 };
        let s2 = Square { side_length: 5*100000};
        s.scale(100000) ;
        assert_eq!(s, s2);

    }
    #[test]
    fn can_rescale_big_circle() {
        let c = Circle { radius: 3.14 };
        let c2 = Circle { radius: 3.14*100000.0};
        s.scale(100000) ;
        assert_eq!(c, c2);
    }
}
