use crate::social_simulation::person::illness_state::IllnessState;
use crate::social_simulation::person::position::Position;
use crate::social_simulation::person::vec2d::Vec2D;
use crate::social_simulation::person::Person;
use crate::social_simulation::world::World;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::collections::VecDeque; // Utilisation d'une queue circulaire

pub mod person;
pub mod world;

const MAX_HISTORY_LENGTH: usize = 10_000; // Limite la taille de l'historique

pub struct SocialSimulation {
    pub population: Vec<Person>,
    pub world: World,
    pub infection_percent: f32,
    pub recovery_chances: f32,
    pub mortality: f32,
    pub total_infected: u32,
    pub total_healthy: u32,
    pub total_dead: u32,
    pub total_recovered: u32,
    pub total_population: u32,
    pub total_masks: u32,
    pub total_vaccinated: u32,
    pub ratio_infected: f32,
    pub history_healthy: VecDeque<u32>, // Historique des personnes saines
    pub history_infected: VecDeque<u32>, // Historique des personnes infectées
    pub history_dead: VecDeque<u32>,     // Historique des personnes mortes
    pub history_recovered: VecDeque<u32>, // Historique des personnes rétablies
    pub history_steps: VecDeque<u32>,    // Historique des étapes (pour l'axe des x)
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
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: SimulationConfig = serde_json::from_str(&contents)?;
    Ok(config)
}

impl SocialSimulation {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = load_config("config.json")?;

        let world = config.world;
        let total_population = config.total_population;
        let infection_percent = config.infection_percent / 100.0;
        let recovery_chances = config.recovery_chances / 100.0;
        let mortality = config.mortality / 100.0;

        let mut rng1 = rand::thread_rng();
        let mut rng2 = rand::thread_rng();

        let mut popu: Vec<Person> = vec![];

        for _ in 0..total_population - 1 {
            let position = Position::new(
                rng1.gen_range(0..world.width) as f64,
                rng2.gen_range(0..world.height) as f64,
            );
            let velocity = Vec2D {
                vx: rng1.gen_range(-1.0..=1.0),
                vy: rng2.gen_range(-1.0..=1.0),
            };
            let person = Person::new(
                world.clone(),
                position,
                velocity,
                IllnessState::Healthy,
                5.0,
            );
            popu.push(person);
        }

        let position = Position::new(
            rng1.gen_range(0..world.width) as f64,
            rng2.gen_range(0..world.height) as f64,
        );
        let velocity = Vec2D {
            vx: rng1.gen_range(-1.0..=1.0),
            vy: rng2.gen_range(-1.0..=1.0),
        };
        let person = Person::new(
            world.clone(),
            position,
            velocity,
            IllnessState::Infected,
            5.0,
        );
        popu.push(person);

        Ok(SocialSimulation {
            population: popu,
            world,
            infection_percent,
            recovery_chances,
            mortality,
            total_infected: 0,
            total_healthy: 0,
            total_dead: 0,
            total_recovered: 0,
            total_population,
            total_masks: 0,
            total_vaccinated: 0,
            ratio_infected: 0.0,
            history_healthy: VecDeque::with_capacity(MAX_HISTORY_LENGTH),
            history_infected: VecDeque::with_capacity(MAX_HISTORY_LENGTH),
            history_dead: VecDeque::with_capacity(MAX_HISTORY_LENGTH),
            history_recovered: VecDeque::with_capacity(MAX_HISTORY_LENGTH),
            history_steps: VecDeque::with_capacity(MAX_HISTORY_LENGTH),
        })
    }

    pub fn infect_someone(&mut self) {
        for index in 0..self.population.len() {
            if self.population[index].state == IllnessState::Healthy {
                self.population[index].state = IllnessState::Infected;
                break;
            }
        }
    }

    pub fn count(&mut self) {
        let mut total_dead = 0;
        let mut total_recovered = 0;
        let mut total_healthy = 0;
        let mut total_infected = 0;
        let mut total_vaccinated = 0;
        let mut total_masks = 0;

        for person in &self.population {
            match person.state {
                IllnessState::Dead => total_dead += 1,
                IllnessState::Recovered => total_recovered += 1,
                IllnessState::Healthy => total_healthy += 1,
                IllnessState::Infected => total_infected += 1,
            }
            if person.wear_mask != 1.0 {
                total_masks += 1;
            }
            if person.vaccinated != 1.0 {
                total_vaccinated += 1;
            }
        }

        self.total_dead = total_dead;
        self.total_recovered = total_recovered;
        self.total_healthy = total_healthy;
        self.total_infected = total_infected;
        self.total_vaccinated = total_vaccinated;
        self.total_masks = total_masks;
        self.ratio_infected = self.total_infected as f32 / (self.total_population - total_dead) as f32;
    }

    pub fn step(&mut self) {
        let pop_snapshot = self.population.clone();
        for index in 0..self.population.len() {
            // Tous ces appels de fonctions auraient pû être fait dans la fonction step de la structure Person pour réduire la dépendance entre SocialSimulation et Person
            self.population[index].step();
            self.population[index].gets_infected(self.infection_percent, &pop_snapshot);
            self.population[index].dies(self.mortality);
            self.population[index].recovers(self.recovery_chances);
            self.population[index].decides_to_wear_mask(&pop_snapshot, self.ratio_infected);
            self.population[index].decides_to_vaccine(self.ratio_infected);
        }

        self.count();

        // Ajouter les données à l'historique
        self.history_healthy.push_back(self.total_healthy);
        self.history_infected.push_back(self.total_infected);
        self.history_dead.push_back(self.total_dead);
        self.history_recovered.push_back(self.total_recovered);
        self.history_steps.push_back(self.history_steps.len() as u32 + 1);

        // Limiter la taille de l'historique
        if self.history_steps.len() > MAX_HISTORY_LENGTH {
            self.history_healthy.pop_front();
            self.history_infected.pop_front();
            self.history_dead.pop_front();
            self.history_recovered.pop_front();
            self.history_steps.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_creation() {
        // Crée ou utilise un fichier de configuration (fichier fictif ici)
        let config_path = "config.json";
        let config_contents = r#"{
            "world": {"width": 100, "height": 100},
            "total_population": 100,
            "infection_percent": 20.0,
            "recovery_chances": 10.0,
            "mortality": 5.0
        }"#;

        // Écrit un fichier temporaire pour les tests
        std::fs::write(config_path, config_contents).unwrap();

        // Charge la simulation
        let sim = SocialSimulation::new().unwrap();

        // Vérifie les valeurs chargées depuis le fichier de configuration
        assert_eq!(sim.world.width, 100);
        assert_eq!(sim.world.height, 100);
        assert_eq!(sim.total_population, 100);
        assert!((sim.infection_percent - 0.2).abs() < f32::EPSILON); // Infection à 20%
        assert!((sim.recovery_chances - 0.1).abs() < f32::EPSILON); // Récupération à 10%
        assert!((sim.mortality - 0.05).abs() < f32::EPSILON); // Mortalité à 5%

        // Nettoie le fichier après le test
        std::fs::remove_file(config_path).unwrap();
    }
}

#[test]
fn test_infect_someone() {
    let config_path = "config_infect.json";
    let config_contents = r#"{
            "world": {"width": 100, "height": 100},
            "total_population": 100,
            "infection_percent": 0.0,
            "recovery_chances": 0.0,
            "mortality": 0.0
        }"#;

    // Écrit un fichier temporaire pour les tests
    std::fs::write(config_path, config_contents).unwrap();

    let mut sim = SocialSimulation::new().unwrap();

    // Initialement, seuls 1 personne est infectée
    sim.count();
    assert_eq!(sim.total_infected, 1);

    // Infecte une personne
    sim.infect_someone();

    // Une personne supplémentaire devrait maintenant être infectée
    sim.count();
    assert_eq!(sim.total_infected, 2); // L'infection total doit passer à 2
}

#[test]
fn test_count() {
    let config_path = "config.json";
    let config_contents = r#"{
            "world": {"width": 100, "height": 100},
            "total_population": 100,
            "infection_percent": 0.0,
            "recovery_chances": 0.0,
            "mortality": 0.0
        }"#;

    // Écrit un fichier temporaire pour les tests
    std::fs::write(config_path, config_contents).unwrap();

    let mut sim = SocialSimulation::new().unwrap();
    sim.step();

    // Compte initial
    sim.count();

    assert!(sim.total_healthy > 0); // Assure qu'il y a des personnes saines
    assert_eq!(sim.total_infected, 1); // Une seule personne infectée initialement
    assert_eq!(sim.total_dead, 0); // Personne n'est mort initialement
    assert_eq!(sim.total_recovered, 0); // Pas encore de guérison
    assert_eq!(sim.total_vaccinated, 0); // Pas de vaccinations au début
    assert_eq!(sim.total_masks, 0); // Aucun masque porté initialement
}


#[test]
fn test_radius_of_infection() {
    let config_path = "config.json";
    let config_contents = r#"{
            "world": {"width": 10, "height": 10},
            "total_population": 100,
            "infection_percent": 1000.0,
            "recovery_chances": 0.0,
            "mortality": 0.0
        }"#;

    // Écrit un fichier temporaire pour les tests
    std::fs::write(config_path, config_contents).unwrap();

    let mut sim = SocialSimulation::new().unwrap();

    // Configure une population minimale avec une seule personne proche d'une infectée
    sim.population.clear();

    let world = sim.world.clone();
    let healthy_position = Position::new(10.0, 10.0);
    let infected_position = Position::new(12.0, 10.0); // Dans le rayon (rayon: 5.0)

    let healthy_person = Person::new(world.clone(), healthy_position, Vec2D { vx: 0.0, vy: 0.0 }, IllnessState::Healthy, 5.0);
    let infected_person = Person::new(world, infected_position, Vec2D { vx: 0.0, vy: 0.0 }, IllnessState::Infected, 5.0);

    sim.population.push(healthy_person);
    sim.population.push(infected_person);

    // Lance une étape pour vérifier si la personne saine devient infectée
    sim.step();

    // Compte les résultats
    sim.count();

    assert_eq!(sim.total_infected, 2);
    assert_eq!(sim.total_healthy, 0); // Tous les individus devraient être infectés
}

#[test]
fn test_simulation_history_length() {
    let mut sim = SocialSimulation::new().unwrap();

    // Fais avancer plusieurs étapes
    for _ in 0..MAX_HISTORY_LENGTH + 100 {
        sim.step();
    }

    // Vérifie que l'historique ne dépasse pas la longueur maximale
    assert_eq!(sim.history_steps.len(), MAX_HISTORY_LENGTH);
    assert_eq!(sim.history_healthy.len(), MAX_HISTORY_LENGTH);
    assert_eq!(sim.history_infected.len(), MAX_HISTORY_LENGTH);
    assert_eq!(sim.history_dead.len(), MAX_HISTORY_LENGTH);
    assert_eq!(sim.history_recovered.len(), MAX_HISTORY_LENGTH);
}