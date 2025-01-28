pub fn add(left: usize, right: usize) -> usize {
    left + right
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
}
