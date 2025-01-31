struct Vec2d {
    x: f64,
    y: f64,
}
struct Point {
    x: f64,
    y: f64,
}

impl Point{
    fn null() -> Self{
        Point{ x: 0.0, y: 0.0 }
    }
    fn new(abs : f64, ord : f64) -> Self{
        Point{ x: abs, y: ord }
    }
}

impl Vec2d {
    fn null() -> Self{
        Vec2d{ x: 0.0, y: 0.0 }
    }

    fn new(abs : f64, ord : f64) -> Self{
        Vec2d{ x: abs, y: ord }
    }

    fn from_points(point1 : Point, point2 : Point) -> Self{
        Vec2d{ x: point1.x - point2.x , y: point1.y - point2.y }
    }




}

#[cfg(test)]
mod tests {
    use std::ptr::null;
    use super::*;
    #[test]
    fn create_vec2d() {
        let vec2d1 = Vec2d{
            x : 1.0,
            y : 16.0,

        };
    }
    #[test]
    fn create_point() {
        let vec2d1 = Vec2d{
            x : 1.0,
            y : 16.0,

        };
    }

    #[test]
    fn vec2d_is_null() {
        let vec2d1 = Vec2d::null();
        assert_eq!(vec2d1.x,0.0);
        assert_eq!(vec2d1.y,0.0);

    }

    #[test]
    fn create_vec2d_param() {
        let vec2d1 = Vec2d::new(65.1,9.0);
        assert_eq!(vec2d1.x,65.1);
        assert_eq!(vec2d1.y,9.0);

    }

    #[test]
    fn get_vec_from_points() {
        let point1 = Point{x:8.0,y:9.0};
        let point2 = Point{x:7.0,y:7.0};

        let vec2d1 = Vec2d::from_points(point1,point2);
        assert_eq!(vec2d1.x,1.0);
        assert_eq!(vec2d1.y,2.0);

    }
    #[test]
    fn point_is_origin() {
        let point1 = Point::null();
        assert_eq!(point1.x,0.0);
        assert_eq!(point1.y,0.0);

    }
    #[test]
    fn create_new_point() {
        let point1 = Point::null();
        assert_eq!(point1.x,0.0);
        assert_eq!(point1.y,0.0);

    }

    #[test]
    fn add_vec() {
        let vec1 = Vec2d::new(8.0, 9.0);
        let vec2 = Vec2d::new(7.0, 7.0);
        let result = vec1.add(vec2);
        assert_eq!(result.x, 15.0);
        assert_eq!(result.y, 16.0);
    }
}
