struct Vec2d {
    x: f64,
    y: f64,
}

struct Point {
    x: f64,
    y: f64,
}

impl Vec2d {
    fn null() -> Self{
        Vec2d{ x: 0.0, y: 0.0 }
    }

    fn new(abs : f64, ord : f64) -> Self{
        Vec2d{ x: abs, y: ord }
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


}
