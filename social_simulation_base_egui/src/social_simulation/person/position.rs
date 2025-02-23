use crate::social_simulation::person::vec2d::Vec2D;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    /// Coordonnée X du vecteur.
    pub x: f64,
    /// Coordonnée Y du vecteur.
    pub y: f64,
}

impl Position {
    pub fn add(&self, velocity: Vec2D) -> Position {
        Position {
            x: self.x + velocity.vx,
            y: self.y + velocity.vy,
        }
    }

    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::social_simulation::person::vec2d::Vec2D;

    #[test]
    fn test_position_creation() {
        // Vérifie si la construction d'une position fonctionne comme prévu
        let pos = Position::new(10.0, 20.0);
        assert_eq!(pos.x, 10.0);
        assert_eq!(pos.y, 20.0);
    }

    #[test]
    fn test_position_add_velocity() {
        // Vérifie si l'addition d'une vitesse modifie correctement les coordonnées de la position
        let pos = Position::new(5.0, 10.0);
        let velocity = Vec2D { vx: 3.0, vy: -2.0 }; // Vecteur de déplacement (vitesse)
        let new_pos = pos.add(velocity);

        // Vérifie la nouvelle position après l'addition
        assert_eq!(new_pos.x, 8.0);
        assert_eq!(new_pos.y, 8.0);
    }

    #[test]
    fn test_position_add_zero_velocity() {
        // Vérifie qu'ajouter une vitesse nulle ne modifie pas la position
        let pos = Position::new(3.0, 4.0);
        let velocity = Vec2D { vx: 0.0, vy: 0.0 }; // Vecteur nul
        let new_pos = pos.add(velocity);

        // Vérifie que la position reste inchangée
        assert_eq!(new_pos.x, pos.x);
        assert_eq!(new_pos.y, pos.y);
    }

    #[test]
    fn test_position_negative_velocity() {
        // Vérifie avec une vitesse ayant des composantes négatives
        let pos = Position::new(7.0, -3.0);
        let velocity = Vec2D { vx: -2.0, vy: -6.0 }; // Vecteur vitesse négatif
        let new_pos = pos.add(velocity);

        // Coordonnées devraient se déplacer vers des valeurs plus négatives
        assert_eq!(new_pos.x, 5.0);
        assert_eq!(new_pos.y, -9.0);
    }

    #[test]
    fn test_position_floating_point_accuracy() {
        // Vérifie le comportement avec des chiffres flottants
        let pos = Position::new(1.1, 2.2);
        let velocity = Vec2D { vx: 0.4, vy: 0.3 };
        let new_pos = pos.add(velocity);

        // Vérifie les nouveaux résultats avec une précision acceptable
        assert!((new_pos.x - 1.5).abs() < f64::EPSILON);
        assert!((new_pos.y - 2.5).abs() < f64::EPSILON);
    }
}