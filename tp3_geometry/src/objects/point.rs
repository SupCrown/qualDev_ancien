use super::vec2d::Vec2D;

/// Représente un point dans un espace 2D.
///
/// # Examples
/// ```
/// use tp3_geometry::objects::point::Point;
///
/// let p = Point::origin();
/// assert_eq!(p.x, 0.0);
/// assert_eq!(p.y, 0.0);
///
/// let p2 = Point::new(3.0, 4.0);
/// assert_eq!(p2.x, 3.0);
/// assert_eq!(p2.y, 4.0);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Point {
    /// Coordonnée X du point.
    pub x: f64,
    /// Coordonnée Y du point.
    pub y: f64,
}

impl Point {
    /// Crée un point à l'origine `(0.0, 0.0)`.
    ///
    /// # Examples
    /// ```
    /// use tp3_geometry::objects::point::Point;
    ///
    /// let p = Point::origin();
    /// assert_eq!(p.x, 0.0);
    /// assert_eq!(p.y, 0.0);
    /// ```
    pub fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    /// Crée un nouveau point à partir des coordonnées `(x, y)`.
    ///
    /// # Examples
    /// ```
    /// use tp3_geometry::objects::point::Point;
    ///
    /// let p = Point::new(3.0, 4.0);
    /// assert_eq!(p.x, 3.0);
    /// assert_eq!(p.y, 4.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// Translate le point selon un vecteur donné.
    ///
    /// # Examples
    /// ```
    /// use tp3_geometry::objects::point::Point;
    /// use tp3_geometry::objects::vec2d::Vec2D;
    ///
    /// let p = Point::new(1.0, 2.0);
    /// let v = Vec2D::new(3.0, 4.0);
    ///
    /// let p_translated = p.translate(v);
    /// assert_eq!(p_translated.x, 4.0);
    /// assert_eq!(p_translated.y, 6.0);
    /// ```
    pub fn translate(self, vec2d: Vec2D) -> Self {
        Self { x: self.x + vec2d.x, y: self.y + vec2d.y }
    }
}