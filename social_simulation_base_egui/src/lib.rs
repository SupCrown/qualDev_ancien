mod app;
mod person;

use eframe::{App, CreationContext};
use rand::Rng;
pub struct SocialSimulation {
    pub person_positions: Vec<(f32, f32)>,
    pub person_speeds: Vec<(f32, f32)>,
    pub person_radius: f32,
    pub steps_per_frame: u32,
}

impl SocialSimulation {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut entities_positions = Vec::new();
        let mut entities_speeds = Vec::new();

        let mut rng = rand::thread_rng();
        for _ in 0..200 {
            let position = (rng.gen_range(0..800) as f32, rng.gen_range(0..600) as f32);
            let speed = (
                rng.gen_range(-1.0..=1.0) as f32,
                rng.gen_range(-1.0..=1.0) as f32,
            );
            entities_positions.push(position);
            entities_speeds.push(speed);
        }

        SocialSimulation {
            person_positions: entities_positions,
            person_speeds: entities_speeds,
            person_radius: 3.0,
            steps_per_frame: 1,
        }
    }
}
