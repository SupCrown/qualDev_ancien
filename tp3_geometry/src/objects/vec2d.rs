use std::f64::consts::PI;
use super::point::Point;

/// Représente un vecteur 2D avec diverses opérations mathématiques.
///
/// # Exemples
/// ```
/// use tp3_geometry::objects::vec2d::Vec2D;
///
/// let v = Vec2D::new(3.0, 4.0);
/// assert_eq!(v.x, 3.0);
/// assert_eq!(v.y, 4.0);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Vec2D {
    /// Coordonnée X du vecteur.
    pub x: f64,
    /// Coordonnée Y du vecteur.
    pub y: f64,
}

impl Vec2D {
    /// Crée un vecteur nul `(0.0, 0.0)`.
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let v = Vec2D::null();
    /// assert_eq!(v.x, 0.0);
    /// assert_eq!(v.y, 0.0);
    /// ```
    pub fn null() -> Self {
        Vec2D { x: 0.0, y: 0.0 }
    }

    /// Crée un nouveau vecteur avec les coordonnées `(x, y)`.
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let v = Vec2D::new(3.0, 4.0);
    /// assert_eq!(v.x, 3.0);
    /// assert_eq!(v.y, 4.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        Vec2D { x, y }
    }

    /// Crée un vecteur à partir de deux points.
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::{point::Point, vec2d::Vec2D};
    ///
    /// let p1 = Point { x: 1.0, y: 1.0 };
    /// let p2 = Point { x: 4.0, y: 5.0 };
    ///
    /// let v = Vec2D::from_points(p1, p2);
    /// assert_eq!(v.x, -3.0);
    /// assert_eq!(v.y, -4.0);
    /// ```
    pub fn from_points(p1: Point, p2: Point) -> Self {
        Vec2D {
            x: p1.x - p2.x,
            y: p1.y - p2.y,
        }
    }

    /// Additionne deux vecteurs.
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let v1 = Vec2D::new(1.0, 2.0);
    /// let v2 = Vec2D::new(3.0, 4.0);
    ///
    /// let v3 = v1.add(v2);
    /// assert_eq!(v3.x, 4.0);
    /// assert_eq!(v3.y, 6.0);
    /// ```
    pub fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Retourne la norme du vecteur.
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let v = Vec2D::new(3.0, 4.0);
    /// assert_eq!(v.norm(), 5.0);
    /// ```
    pub fn norm(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Applique une homothétie (`scale`) au vecteur.
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let v = Vec2D::new(1.0, 2.0).scale(2.0);
    /// assert_eq!(v.x, 2.0);
    /// assert_eq!(v.y, 4.0);
    /// ```
    pub fn scale(self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    /// Normalise le vecteur (le rend de norme `1`).
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let v = Vec2D::new(3.0, 4.0).normalize();
    /// assert!((v.norm() - 1.0).abs() < 1e-6);
    /// ```
    pub fn normalize(self) -> Self {
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

    /// Applique une rotation d'un angle en degrés.
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let v = Vec2D::new(1.0, 0.0).rotate(90.0);
    /// assert!((v.x - 0.0).abs() < 1e-6);
    /// assert!((v.y - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate(self, angle: f64) -> Self {
        let rad = angle * PI / 180.0;
        Self {
            x: self.x * rad.cos() - self.y * rad.sin(),
            y: self.x * rad.sin() + self.y * rad.cos(),
        }
    }

    /// Retourne l’angle entre deux vecteurs en degrés.
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let v1 = Vec2D::new(1.0, 0.0);
    /// let v2 = Vec2D::new(0.0, 1.0);
    ///
    /// assert!((v1.angle(v2) - 90.0).abs() < 1e-6);
    /// ```
    pub fn angle(self, other: Self) -> f64 {
        let dot_product = self.dot(other);
        let norms = self.norm() * other.norm();
        if norms == 0.0 {
            0.0
        } else {
            (dot_product / norms).acos() * 180.0 / PI
        }
    }

    /// Produit scalaire entre deux vecteurs.
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let v1 = Vec2D::new(1.0, 2.0);
    /// let v2 = Vec2D::new(3.0, 4.0);
    ///
    /// assert_eq!(v1.dot(v2), 11.0);
    /// ```
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Vérifie si deux vecteurs sont colinéaires.
    ///
    /// # Exemples
    /// ```
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let v1 = Vec2D::new(2.0, 4.0);
    /// let v2 = Vec2D::new(1.0, 2.0);
    ///
    /// assert!(v1.colinear(v2));
    /// ```
    pub fn colinear(self, other: Self) -> bool {
        (self.x * other.y - self.y * other.x).abs() < 1e-6
    }
}
