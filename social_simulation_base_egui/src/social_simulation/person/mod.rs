pub mod illness_state;
pub mod position;
pub mod vec2d;

use crate::social_simulation::person::illness_state::IllnessState;
use crate::social_simulation::person::position::Position;
use crate::social_simulation::person::vec2d::Vec2D;
use crate::social_simulation::world::World;
use rand::Rng;

#[derive(Clone)]
pub struct Person {
    pub world: World,
    pub position: Position,
    pub velocity: Vec2D,
    pub state: IllnessState,
    pub contamination_radius: f64,
    pub wear_mask: f32,  //equal 1 if false, equal 0.35 if true
    pub vaccinated: f32, //equal 1 if false, equal 0.60 if true
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
            wear_mask: 1.0,
            vaccinated: 1.0,
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
            }
        }
    }

    pub fn put_mask_on(&mut self) {
        // is not enabled if already vaccinated
            self.wear_mask = 0.75;
    }

    pub fn put_mask_off(&mut self) {
        self.wear_mask = 1.0;
    }
    pub fn vaccine(&mut self) {
        if self.wear_mask != 1.0 {
            self.wear_mask = 1.0;
        }
        self.vaccinated = 0.05;
    }

    pub fn is_someone_near(&self,population: &[Person])-> bool{
        for p in population {
            if self.is_in_radius(p.position) {
                return true;
            }
        }
        false
    }

    pub fn is_someone_infected_near(&self,population: &[Person])-> bool{
        for p in population {
            if self.is_in_radius(p.position) {
                if p.state == IllnessState::Infected {
                    return true;
                }
            }
        }
        false
    }

    // Vérifie si la personne doit être infectée à partir d'un snapshot de la population.
    pub fn gets_infected(&mut self,  infection_percentage: f32, population: &[Person]) {
        let mut rng1 = rand::thread_rng();

            if self.is_someone_infected_near(population) {
                if self.state == IllnessState::Healthy {
                        if rng1.gen_range(0.0..=1.0) < infection_percentage * self.wear_mask * self.vaccinated
                        {
                            self.state = IllnessState::Infected;
                        }
                }else if self.state == IllnessState::Recovered {
                    if rng1.gen_range(0.0..=1.0) < (infection_percentage/2.0) * self.wear_mask * self.vaccinated
                    {
                        self.state = IllnessState::Infected;
                    }
                }
        }
    }

    pub fn dies(&mut self, mortality: f32) {
        let mut rng1 = rand::thread_rng();
        if self.state == IllnessState::Infected {
            if rng1.gen_range(0.0..=1.0) < mortality*self.vaccinated {
                self.state = IllnessState::Dead;
            }
        }
    }

    pub fn decides_to_vaccine(&mut self, ratio_infected: f32) {
        let mut rng1 = rand::thread_rng();
        if self.vaccinated == 1.0 {
                if rng1.gen_range(0.0..=1.0) < 0.1*ratio_infected/300.0 {
                    self.vaccine()
                }
            }
    }

    pub fn decides_to_wear_mask(&mut self,population: &[Person],ratio_infected: f32) {
        let mut rng1 = rand::thread_rng();
        if self.is_someone_near(population) {
            if rng1.gen_range(0.0..=1.0) < 0.30* ratio_infected/100.0{
                self.put_mask_on();
            }
        } else {
            if rng1.gen_range(0.0..=1.0) < 0.90 {
                self.put_mask_off();
            }
        }
    }

    pub fn recovers(&mut self, recovery_chances: f32) {
        let mut rng1 = rand::thread_rng();
        if self.state == IllnessState::Infected {
            if rng1.gen_range(0.0..=1.0) < recovery_chances/self.vaccinated {
                self.state = IllnessState::Recovered;
            }
        }
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::social_simulation::world::World;
    use crate::social_simulation::person::illness_state::IllnessState;
    use crate::social_simulation::person::vec2d::Vec2D;
    use crate::social_simulation::person::position::Position;

    #[test]
    fn test_person_creation() {
        let world = World { width: 100, height: 100 };
        let position = Position::new(10.0, 20.0);
        let velocity = Vec2D { vx: 1.0, vy: 1.5 };
        let state = IllnessState::Healthy;
        let person = Person::new(world.clone(), position, velocity, state, 5.0);

        assert_eq!(person.position.x, 10.0);
        assert_eq!(person.position.y, 20.0);
        assert_eq!(person.velocity.vx, 1.0);
        assert_eq!(person.velocity.vy, 1.5);
        assert_eq!(person.state, IllnessState::Healthy);
        assert_eq!(person.wear_mask, 1.0);
        assert_eq!(person.vaccinated, 1.0);
        assert_eq!(person.contamination_radius, 5.0);
        assert_eq!(person.world.width, 100);
        assert_eq!(person.world.height, 100);
    }
    #[test]
    fn test_person_step() {
        let world = World { width: 50, height: 50 };
        let position = Position::new(48.0, 25.0);
        let velocity = Vec2D { vx: 3.0, vy: 2.0 };
        let mut person = Person::new(world, position, velocity, IllnessState::Healthy, 5.0);

        person.step();

        // Vérifie que la personne se déplace correctement.
        assert_eq!(person.position.x, 0.0); // World wrap-around en X
        assert_eq!(person.position.y, 27.0); // Pas de wrap-around en Y
    }

    #[test]
    fn test_person_step_dead() {
        let world = World { width: 50, height: 50 };
        let position = Position::new(10.0, 20.0);
        let velocity = Vec2D { vx: 1.0, vy: 1.0 };
        let mut person = Person::new(world, position, velocity, IllnessState::Dead, 5.0);

        person.step();

        // Si la personne est morte, elle ne bouge pas !
        assert_eq!(person.position.x, 10.0);
        assert_eq!(person.position.y, 20.0);
    }
    #[test]
    fn test_person_put_mask_on() {
        // Vérifie que mettre un masque modifie correctement wear_mask
        let world = World { width: 100, height: 100 };
        let position = Position::new(0.0, 0.0);
        let velocity = Vec2D { vx: 0.0, vy: 0.0 };
        let mut person = Person::new(world, position, velocity, IllnessState::Healthy, 1.0);

        person.put_mask_on(); // Mets un masque
        assert_eq!(person.wear_mask, 0.75);

        person.put_mask_off(); // Enlève le masque
        assert_eq!(person.wear_mask, 1.0);
    }
    #[test]
    fn test_person_gets_infected() {
        // Prépare une population de personnes, dont une infectée
        let world = World { width: 100, height: 100 };
        let position1 = Position::new(0.0, 0.0);
        let position2 = Position::new(0.5, 0.5); // Proche de position1 (dans le rayon)
        let velocity = Vec2D { vx: 0.0, vy: 0.0 };

        let mut person = Person::new(world.clone(), position1, velocity, IllnessState::Healthy, 1.0);
        let infected_person = Person::new(
            world,
            position2,
            velocity,
            IllnessState::Infected,
            1.0,
        );

        let population = vec![infected_person];

        person.gets_infected(1.0, &population); // Infection 100% probabilité

        assert_eq!(person.state, IllnessState::Infected);
    }
    #[test]
    fn test_is_in_radius() {
        let world = World { width: 50, height: 50 };
        let position1 = Position::new(10.0, 10.0);
        let position2 = Position::new(12.0, 12.0); // Dans le rayon de 3
        let position3 = Position::new(20.0, 20.0); // Hors du rayon de 3

        let person = Person::new(world, position1, Vec2D { vx: 0.0, vy: 0.0 }, IllnessState::Healthy, 3.0);

        assert!(person.is_in_radius(position2)); // Position 2 est dans le rayon
        assert!(!person.is_in_radius(position3)); // Position 3 est hors du rayon
    }
}