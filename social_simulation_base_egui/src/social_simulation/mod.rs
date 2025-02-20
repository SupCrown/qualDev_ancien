use crate::social_simulation::person::Person;
use crate::social_simulation::person::illness_state::IllnessState;
use crate::social_simulation::person::position::Position;
use crate::social_simulation::person::vec2d::Vec2D;
use crate::social_simulation::world::World;
use rand::Rng;

pub mod person;
pub mod world;

pub struct SocialSimulation {
    pub(crate) population: Vec<Person>,
    pub world: World,
    pub person_radius: f32,
    pub steps_per_frame: u32,
    pub infection_percent: f32,
    pub recovery_chances: f32,
    pub mortality: f32,


}

impl SocialSimulation {
    pub fn new(world: World) -> Self {
        let mut rng1 = rand::thread_rng();
        let mut rng2 = rand::thread_rng();

        let mut popu: Vec<Person> = vec![];

        // Crée 200 personnes saines
        for _ in 0..200 {
            let position = Position::new(
                rng1.gen_range(0..world.height) as f64,
                rng2.gen_range(0..world.width) as f64,
            );
            let velocity = Vec2D {
                vx: rng1.gen_range(-1.0..=1.0),
                vy: rng2.gen_range(-1.0..=1.0),
            };
            let person = Person::new(world.clone(), position, velocity, IllnessState::Healthy, 5.0);
            popu.push(person);
        }

        // Ajoute une personne infectée
        let position = Position::new(
            rng1.gen_range(0..world.height) as f64,
            rng2.gen_range(0..world.width) as f64,
        );
        let velocity = Vec2D {
            vx: rng1.gen_range(-1.0..=1.0),
            vy: rng2.gen_range(-1.0..=1.0),
        };
        let person = Person::new(world.clone(), position, velocity, IllnessState::Infected, 5.0);
        popu.push(person);

        SocialSimulation {
            population: popu,
            world,
            person_radius: 0.1,
            steps_per_frame: 1,
            infection_percent: 0.0,
            recovery_chances: 0.0,
            mortality: 0.0,
        }
    }

    pub fn step(&mut self) {
        for person in &mut self.population {
            person.step();
        }

        let pop_snapshot = self.population.clone();
        for person in &mut self.population {
            if person.gets_infected(&pop_snapshot,self.infection_percent) {
                person.state = IllnessState::Infected;
            }
            if person.dies(self.mortality){
                person.state = IllnessState::Dead;
            }
            if person.dies(self.recovery_chances){
                person.state = IllnessState::Recovered;
            }
        }

    }
}