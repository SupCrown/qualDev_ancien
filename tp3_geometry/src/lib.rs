struct Vec2d {
    x: f64,
    y: f64,
}

struct Point {
    x: f64,
    y: f64,
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
        let vec2d1 = Vec2d::vec_null();
        assert_eq!(vec2d1.x,0.0);
        assert_eq!(vec2d1.y,0.0);

    }


}
