use std::fs::File;
use std::io::Read;
use serde_json::Error;
use crate::social_simulation::person::Person;
use crate::social_simulation::person::illness_state::IllnessState;
use crate::social_simulation::person::position::Position;
use crate::social_simulation::person::vec2d::Vec2D;
use crate::social_simulation::world::World;
use rand::Rng;
use serde::{Deserialize, Serialize};

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
    pub total_infected:u32,
    pub total_healthy:u32,
    pub total_dead:u32,
    pub total_recovered:u32,
    pub total_population:u32,

}

#[derive(Debug, Deserialize, Serialize)]
struct SimulationConfig {
    world: World,
    total_population: u32,
    infection_percent: f32,
    recovery_chances: f32,
    mortality: f32,
}

fn load_config(file_path: &str) -> Result<SimulationConfig, Box<dyn std::error::Error>> {
    // Ouvrir le fichier
    let mut file = File::open(file_path)?;

    // Lire le contenu du fichier dans une chaîne de caractères
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Désérialiser le contenu JSON en une instance de SimulationConfig
    let config: SimulationConfig = serde_json::from_str(&contents)?;

    // Retourner la configuration
    Ok(config)
}

impl SocialSimulation {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Charger la configuration depuis le fichier JSON
        let config = load_config("config.json")?;

        let world = config.world;
        let total_population = config.total_population;
        let infection_percent = config.infection_percent;
        let recovery_chances = config.recovery_chances;
        let mortality = config.mortality;

        let mut rng1 = rand::thread_rng();
        let mut rng2 = rand::thread_rng();

        let mut popu: Vec<Person> = vec![];

        for _ in 0..total_population-1 {
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

        Ok(SocialSimulation {
            population: popu,
            world,
            person_radius: 0.1,
            steps_per_frame: 1,
            infection_percent,
            recovery_chances,
            mortality,
            total_infected: 0,
            total_healthy: 0,
            total_dead: 0,
            total_recovered: 0,
            total_population,
        })
    }

    pub fn infect_someone(&mut self) {


        for mut index in 0..self.population.len() {
            if self.population[index].state == IllnessState::Healthy{
                self.population[index].state = IllnessState::Infected;
                break;
            }
        }
    }

    pub fn count(&mut self) {
        let mut total_dead = 0 ;
        let mut total_recovered = 0 ;
        let mut total_healthy = 0 ;
        let mut total_infected = 0 ;

        for person in &self.population {
            if person.state == IllnessState::Dead {
                total_dead += 1;
            }
            if person.state == IllnessState::Recovered {
                total_recovered += 1;
            }
            if person.state == IllnessState::Healthy {
                total_healthy += 1;
            }
            if person.state == IllnessState::Infected {
                total_infected += 1;
            }
        }
        self.total_dead = total_dead;
        self.total_recovered = total_recovered;
        self.total_healthy = total_healthy;
        self.total_infected = total_infected;


    }


    pub fn step(&mut self) {
        let pop_snapshot = self.population.clone();
        for index in 0..self.population.len() {
            self.population[index].step();
            self.count();
            if self.population[index].gets_infected(&pop_snapshot,self.infection_percent) {
                self.population[index].state = IllnessState::Infected;
            }
            if self.population[index].dies(self.mortality){
                self.population[index].state = IllnessState::Dead;
            }
            if self.population[index].dies(self.recovery_chances){
                self.population[index].state = IllnessState::Recovered;
            }
        }


    }
}