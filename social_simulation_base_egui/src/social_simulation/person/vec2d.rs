#[derive(Clone, Copy, Debug)]
pub struct Vec2D {
    /// Coordonnée X du vecteur.
    pub vx: f64,
    /// Coordonnée Y du vecteur.
    pub vy: f64,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2d_creation() {
        // Test basique pour vérifier la création initiale d'un vecteur
        let v = Vec2D { vx: 3.0, vy: 4.0 };
        assert_eq!(v.vx, 3.0);
        assert_eq!(v.vy, 4.0);
    }

    #[test]
    fn test_vec2d_init_with_zero() {
        // Vérifie si on peut initialiser un vecteur nul sans erreur
        let v = Vec2D { vx: 0.0, vy: 0.0 };
        assert_eq!(v.vx, 0.0);
        assert_eq!(v.vy, 0.0);
    }
}