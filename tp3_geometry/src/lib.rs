pub fn add(left: usize, right: usize) -> usize {
    left + right
}
struct Point {
    x1: f64,
    y2: f64,
}
impl Point {
    pub fn new(x1: f64, y2: f64) -> Self {
        Point { x1, y2 }
    }
}

struct Vec2D {
    x: f64,
    y: f64,
}
impl Vec2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2D { x, y }
    }
    pub fn null(x: f64, y: f64) -> Self {
        Vec2D { x, y }
    }
    pub fn from_points(p1:Point(2,5), p2:Point) -> Self{
        Vec2D {p1.x1-p2.x1, p1.y2-p2.y2}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vecteur_works() {
        let vecteur = Vec2D::new(22.3, 36.1);
        assert_eq!(vecteur.x, 22.3);
        assert_eq!(vecteur.y, 36.1);
    }

    #[test]
    fn create_vecteur_null() {
        let vecteur = Vec2D::null(0.0, 0.0);
        assert_eq!(vecteur.x, 0.0);
        assert_eq!(vecteur.y, 0.0);
    }

    #[test]
    fn create_vecteur_new() {
        let f1 = 12.5;
        let f2 = 23.1;
        let vecteur = Vec2D::new(f1, f2);
        assert_eq!(vecteur.x, f1);
        assert_eq!(vecteur.y, f2);
    }

    #[test]
    fn create_vecteur_from_points() {
        let p1 = Point::new(2.3, 6.2);
        let p2 = Point::new(5.9, 9.8);
        let vecteur = Vec2D::from_points(p1, p2);
        assert_eq!(vecteur.p1, p1.x1 - p2.x1);
        assert_eq!(vecteur.p2, p1.y2 - p2.y2);
    }
}
