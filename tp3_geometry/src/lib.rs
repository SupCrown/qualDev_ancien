use std::f64::consts::PI;
#[derive(Clone, Copy, Debug)]
struct Vec2D {
    x: f64,
    y: f64,
}
#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn null() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(abs: f64, ord: f64) -> Self {
        Point { x: abs, y: ord }
    }

    fn translate(self, vec2d: Vec2D) -> Self {
        Self {x :self.x + vec2d.x,
            y: self.y + vec2d.y}
    }
}

impl Vec2D {
    fn null() -> Self {
        Vec2D { x: 0.0, y: 0.0 }
    }

    fn new(abs: f64, ord: f64) -> Self {
        Vec2D { x: abs, y: ord }
    }

    fn from_points(point1: Point, point2: Point) -> Self {
        Vec2D {
            x: point1.x - point2.x,
            y: point1.y - point2.y,
        }
    }

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn norm(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    fn scale(self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
    fn normalize(self) -> Self {
        let norm = self.norm();
        if norm == 0.0 {
            Self::null()
        } else {
            Self {
                x: self.x / norm,
                y: self.y / norm,
            }
        }
    }
    pub fn rotate(self, angle: f64) -> Self {
        let rad = angle * PI / 180.0;
        Self {
            x: self.x * rad.cos() - self.y * rad.sin(),
            y: self.x * rad.sin() + self.y * rad.cos(),
        }
    }
    pub fn angle(self, other: Self) -> f64 {
        let dot_product = self.dot(other);
        let norms = self.norm() * other.norm();
        if norms == 0.0 {
            0.0
        } else {
            (dot_product / norms).acos() * 180.0 / PI
        }
    }
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::null;
    #[test]
    fn create_vec2d() {
        let vec2d1 = Vec2D { x: 1.0, y: 16.0 };
    }
    #[test]
    fn create_point() {
        let vec2d1 = Vec2D { x: 1.0, y: 16.0 };
    }

    #[test]
    fn vec2d_is_null() {
        let vec2d1 = Vec2D::null();
        assert_eq!(vec2d1.x, 0.0);
        assert_eq!(vec2d1.y, 0.0);
    }

    #[test]
    fn create_vec2d_param() {
        let vec2d1 = Vec2D::new(65.1, 9.0);
        assert_eq!(vec2d1.x, 65.1);
        assert_eq!(vec2d1.y, 9.0);
    }

    #[test]
    fn get_vec_from_points() {
        let point1 = Point { x: 8.0, y: 9.0 };
        let point2 = Point { x: 7.0, y: 7.0 };

        let vec2d1 = Vec2D::from_points(point1, point2);
        assert_eq!(vec2d1.x, 1.0);
        assert_eq!(vec2d1.y, 2.0);
    }
    #[test]
    fn point_is_origin() {
        let point1 = Point::null();
        assert_eq!(point1.x, 0.0);
        assert_eq!(point1.y, 0.0);
    }
    #[test]
    fn create_new_point() {
        let point1 = Point::null();
        assert_eq!(point1.x, 0.0);
        assert_eq!(point1.y, 0.0);
    }

    #[test]
    fn add_vec() {
        let vec1 = Vec2D::new(8.0, 9.0);
        let vec2 = Vec2D::new(7.0, 7.0);
        let result = vec1.add(vec2);
        assert_eq!(result.x, 15.0);
        assert_eq!(result.y, 16.0);
    }

    #[test]
    fn vec2d_norm() {
        let vec = Vec2D::new(3.0, 4.0);
        assert_eq!(vec.norm(), 5.0);
    }

    #[test]
    fn vec2d_scale() {
        let vec = Vec2D::new(2.0, 3.0).scale(2.0);
        assert_eq!(vec.x, 4.0);
        assert_eq!(vec.y, 6.0);
    }

    #[test]
    fn vec2d_normalize() {
        let vec = Vec2D::new(3.0, 4.0).normalize();
        assert!((vec.x - 0.6).abs() < 1e-6);
        assert!((vec.y - 0.8).abs() < 1e-6);
    }
    #[test]
    fn vec2d_translate() {
        let vec = Vec2D::new(3.0, 4.0);
        let point = Point { x: 1.0, y: 2.0 };
        let result = point.translate(vec);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }


    #[test]
    fn vec2d_rotate() {
        let vec = Vec2D::new(1.0, 0.0).rotate(90.0);
        assert!((vec.x - 0.0).abs() < 1e-6);
        assert!((vec.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn vec2d_angle() {
        let vec1 = Vec2D::new(1.0, 0.0);
        let vec2 = Vec2D::new(0.0, 1.0);
        assert!((vec1.angle(vec2) - 90.0).abs() < 1e-6);
    }

    #[test]
    fn vec2d_dot() {
        let vec1 = Vec2D::new(1.0, 2.0);
        let vec2 = Vec2D::new(3.0, 4.0);
        assert_eq!(vec1.dot(vec2), 11.0);
    }
}
