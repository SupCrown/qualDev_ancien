use crate::world::World;
use super::person::Person;
struct SocialSimulation{
    population:Vec<Person>,
    world: World,
}

impl SocialSimulation{
    pub fn new(population:Vec<Person>, world: World)-> Self{
        SocialSimulation{population,world}
    }

    pub fn step(&self){
        for mut person in self.population {
            person.step();
        }
    }
}