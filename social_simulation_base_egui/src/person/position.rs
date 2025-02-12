use std::f64::consts::PI;
use crate::person::vec2d::Vec2D;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    /// Coordonnée X du vecteur.
    pub x: f64,
    /// Coordonnée Y du vecteur.
    pub y: f64,
}

impl Position{
    pub fn add(&self, velocity:Vec2D)-> Position{
        Position{ x: self.x + velocity.vx, y: self.y + velocity.vy }
    }
}

