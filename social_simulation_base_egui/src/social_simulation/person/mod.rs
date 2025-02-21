pub mod illness_state;
pub mod position;
pub mod vec2d;

use crate::social_simulation::world::World;
use rand::Rng;
use std::ops::Add;
use crate::social_simulation::person::illness_state::IllnessState;
use crate::social_simulation::person::position::Position;
use crate::social_simulation::person::vec2d::Vec2D;

#[derive(Clone)]
pub struct Person {
    pub world: World,
    pub position: Position,
    pub velocity: Vec2D,
    pub state: IllnessState,
    pub contamination_radius: f64,
}

impl Person {
    pub fn new(
        world: World,
        position: Position,
        velocity: Vec2D,
        state: IllnessState,
        contamination_radius: f64,
    ) -> Self {
        Person {
            world,
            position,
            velocity,
            state,
            contamination_radius,
        }
    }

    // Met à jour la position de la personne
    pub fn step(&mut self) {
        if self.state != IllnessState::Dead {
        self.position = self.position.add(self.velocity);

        if self.position.x > self.world.width as f64 {
            self.position.x = 0.0;
        }
        if self.position.x < 0.0 {
            self.position.x = self.world.width as f64;
        }
        if self.position.y > self.world.height as f64 {
            self.position.y = 0.0;
        }
        if self.position.y < 0.0 {
            self.position.y = self.world.height as f64;
        }}
    }

    // Vérifie si la personne doit être infectée à partir d'un snapshot de la population.
    pub fn gets_infected(&mut self, population: &[Person], infection_percentage:f32) -> bool {
        let mut rng1 = rand::thread_rng();

        for person in population.iter() {
            if person.state == IllnessState::Infected {
                if self.state == IllnessState::Healthy {
                    if self.is_in_radius(person.position) {
                    if rng1.gen_range(0.0..=1.0) < infection_percentage {
                        return true;
                    }
                }}

            }
        }
        false
    }

    pub fn dies(&mut self, mortality:f32) -> bool {
        let mut rng1 = rand::thread_rng();
        if self.state == IllnessState::Infected {
            if rng1.gen_range(0.0..=1.0) < mortality {
                return true
            }
        }
        false
    }

    pub fn recovers(&mut self, recovery_chances:f32) -> bool {
        let mut rng1 = rand::thread_rng();
        if self.state == IllnessState::Infected {
            if rng1.gen_range(0.0..=1.0) < recovery_chances {
                return true
            }
        }
        false
    }



    pub fn is_in_radius(&self, position: Position) -> bool {
        // Vérifie si la position en paramètre est dans le rayon de contamination de la personne
        if self.position.x < position.x + self.contamination_radius
            && self.position.x > position.x - self.contamination_radius
        {
            if self.position.y < position.y + self.contamination_radius
                && self.position.y > position.y - self.contamination_radius
            {
                return true;
            }
        }
        false
    }
}
