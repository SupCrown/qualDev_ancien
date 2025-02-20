use egui::{Sense, Vec2};
use rand::Rng;
use person::illness_state::IllnessState;
use person::Person;
use person::position::Position;
use person::vec2d::Vec2D;
use world::World;

pub mod person;
pub mod world;

pub struct SocialSimulation{
    pub(crate) population:Vec<Person>,
    pub world: World,
    pub person_radius : f32,
    pub steps_per_frame : u32
}

impl SocialSimulation{
    pub fn new(world: World)-> Self{
        let mut rng1 = rand::thread_rng();
        let mut rng2 = rand::thread_rng();


        let mut popu:Vec<Person> = vec![];

        for _ in 0..200 {
           let position:Position = Position::new(rng1.gen_range(0..world.height) as f64, rng2.gen_range(0..world.width) as f64);
            let velocity:Vec2D = Vec2D {
                vx:rng1.gen_range( - 1.0..=1.0) as f64,
                vy:rng2.gen_range( - 1.0..=1.0) as f64,
            };
                let person = Person::new(world, position, velocity, IllnessState::Healthy, 5.0);

            popu.push(person);
        }
        let position:Position = Position::new(rng1.gen_range(0..world.height) as f64, rng2.gen_range(0..world.width) as f64);
        let velocity:Vec2D = Vec2D {
            vx:rng1.gen_range( - 1.0..=1.0) as f64,
            vy:rng2.gen_range( - 1.0..=1.0) as f64,
        };
        let person = Person::new(world, position, velocity, IllnessState::Infected, 5.0);
        popu.push(person);


        SocialSimulation{
            population:popu,
            world,
            person_radius:0.1,
            steps_per_frame:1,
        }
    }

    pub fn step(&mut self){
        for person in &mut self.population {
            person.step();
        }

    }
}

