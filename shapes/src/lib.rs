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
}
