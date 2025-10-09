use shapes::Circle;
use shapes::Rectangle;
use shapes::Square;

    #[test]
    fn can_create_circle() {
        let c = Circle { radius: 3.14 };
        assert_eq!(c.radius, 3.14);
    }
    #[test]
    fn can_create_square() {
        let s = Square { side_length: 4.0 };
        assert_eq!(s.side_length, 4.0);
    }
    #[test]
    fn can_create_rectangle() {
        let r = Rectangle { length: 5.0, width: 6.0 };
        assert_eq!(r.length, 5.0);
        assert_eq!(r.width, 6.0);

    }
    #[test]
    fn can_calculate_circle_area() {
        let c = Circle { radius: 3.14 };
        let area = c.area();
        assert_eq!(area, 30.974846927333925);

    }
    #[test]
    fn can_calculate_square_area() {
        let s = Square { side_length: 5.0};
        let area = s.area();
        assert_eq!(area, 5.0 * 5.0);

    }
    #[test]
    fn can_calculate_rectangle_area() {
        let r = Rectangle { length: 5.0, width: 6.0 };
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
        let s = Square { side_length: 5.0 };
        let perimeter = s.perimeter();
        assert_eq!(perimeter, 5.0 * 4.0);

    }
    #[test]
    fn can_calculate_rectangle_perimeter() {
        let r = Rectangle { length: 5.0, width: 6.0 };
        let perimeter = r.perimeter();
        assert_eq!(perimeter, 6.0 *2.0 + 2.0 * 5.0);

    }

    #[test]
    fn can_rescale_rectangle() {
        let mut r = Rectangle { length: 5.0, width: 6.0 };
        let r2 = Rectangle { length: 10.0, width: 12.0 };
        r.scale(2.0) ;
        assert_eq!(r, r2);

    }
    #[test]
    fn can_rescale_square() {
        let mut s = Square { side_length: 5.0 };
        let s2 = Square { side_length: 10.0};
        s.scale(2.0) ;
        assert_eq!(s, s2);

    }
    #[test]
    fn can_rescale_circle() {
        let mut c = Circle { radius: 3.14 };
        let c2 = Circle { radius: 3.14*2.0};
        c.scale(2.0) ;
        assert_eq!(c, c2);
    }
    #[test]
    fn can_rescale_zero_rectangle() {
        let mut r = Rectangle { length: 5.0, width: 6.0 };
        let r2 = Rectangle { length: 5.0*0.0, width: 6.0*0.0 };
        r.scale(0.0) ;
        assert_eq!(r, r2);

    }
    #[test]
    fn can_rescale_zero_square() {
        let mut s = Square { side_length: 5.0 };
        let s2 = Square { side_length: 5.0*0.0};
        s.scale(0.0) ;
        assert_eq!(s, s2);

    }
    #[test]
    fn can_rescale_zero_circle() {
        let mut c = Circle { radius: 3.14 };
        let c2 = Circle { radius: 3.14*0.0};
        c.scale(0.0) ;
        assert_eq!(c, c2);
    }
    #[test]
    fn can_rescale_big_rectangle() {
        let mut r = Rectangle { length: 5.0, width: 6.0 };
        let r2 = Rectangle { length: 5.0*100000.0, width: 6.0*100000.0 };
        r.scale(100000.0) ;
        assert_eq!(r, r2);

    }
    #[test]
    fn can_rescale_big_square() {
        let mut s = Square { side_length: 5.0 };
        let s2 = Square { side_length: 5.0*100000.0};
        s.scale(100000.0) ;
        assert_eq!(s, s2);

    }
    #[test]
    fn can_rescale_big_circle() {
        let mut c = Circle { radius: 3.14 };
        let c2 = Circle { radius: 3.14*100000.0};
        c.scale(100000.0) ;
        assert_eq!(c, c2);
    }
    #[test]
    fn can_rescale_reduce_rectangle() {
        let mut r = Rectangle { length: 5.0, width: 6.0 };
        let r2 = Rectangle { length: 5.0 * 0.5, width: 6.0 * 0.5};
        r.scale(0.5) ;
        assert_eq!(r, r2);

    }
    #[test]
    fn can_rescale_reduce_square() {
        let mut s = Square { side_length: 5.0 };
        let s2 = Square { side_length: 5.0 * 0.5 };
        s.scale(0.5) ;
        assert_eq!(s, s2);

    }
    #[test]
    fn can_rescale_reduce_circle() {
        let mut c = Circle { radius: 3.14 };
        let c2 = Circle { radius: 3.14*0.5};
        c.scale(0.5) ;
        assert_eq!(c, c2);
    }

