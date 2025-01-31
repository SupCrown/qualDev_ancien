pub mod objects;

use crate::objects::point::Point;
use crate::objects::vec2d::Vec2D;
use std::f64::consts::PI;

fn main() {
    let mut p = Point::origin();
    let v = Vec2D::new(12.3, 45.6);

    let transformed_v = v.scale(3.0).rotate(PI).add(Vec2D::new(1.0, 1.0));
    p.translate(transformed_v);
}


#[cfg(test)]
mod tests {
    use super::*;
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
        let point1 = Point::origin();
        assert_eq!(point1.x, 0.0);
        assert_eq!(point1.y, 0.0);
    }
    #[test]
    fn create_new_point() {
        let point1 = Point::origin();
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


    #[test]
    fn vec2d_colinear() {
        let vec1 = Vec2D::new(2.0, 4.0);
        let vec2 = Vec2D::new(1.0, 2.0);
        assert!(vec1.colinear(vec2));
    }

    #[test]
    fn vec2d_translate_by_null() {
        let point = Point::new(3.0, 4.0);
        let result = point.translate(Vec2D::null());
        assert_eq!(result.x, 3.0 );
        assert_eq!(result.y, 4.0 );
    }

    #[test]
    fn vec2d_rotate_360() {
        let vec = Vec2D::new(1.0, 0.0).rotate(360.0);
        assert!((vec.x - 1.0).abs() < 1e-6);
        assert!((vec.y - 0.0).abs() < 1e-6);
    }
}