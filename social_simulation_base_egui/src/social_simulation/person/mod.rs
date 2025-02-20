use crate::social_simulation::world::World;
use egui::scroll_area::State;
use egui::Slider;
use illness_state::IllnessState;
use position::Position;
use rand::Rng;
use std::cmp::PartialEq;
use std::ops::Add;
use vec2d::Vec2D;

pub mod illness_state;
pub mod position;
pub mod vec2d;

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

    pub fn step(&mut self) {
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
        }
    }
    pub fn who_is_in_radius(&mut self, mut population: Vec<Person>) -> Vec<Person> {
        let mut rng1 = rand::thread_rng();

        for mut person in &population {
            if self.is_in_radius(person.position) {
                if person.state == IllnessState::Infected {
                    if rng1.gen_range(0.0..=1.0) < 0.15 {
                        self.state = IllnessState::Infected
                    }
                }
            }
        }
        population
    }

    pub fn is_in_radius(&self, position: Position) -> bool {
        if self.position.x < position.x + self.contamination_radius
            && self.position.x > position.x - self.contamination_radius
        {
            if self.position.y < position.y + self.contamination_radius
                && self.position.y > position.y - self.contamination_radius
            {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
