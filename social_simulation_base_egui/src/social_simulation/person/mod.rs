use egui::scroll_area::State;
use egui::Slider;
use position::Position;
use vec2d::Vec2D;
use crate::social_simulation::world::World;
use illness_state::IllnessState;

pub mod illness_state;
pub mod vec2d;
pub mod position;


pub struct Person{
    pub world: World,
    pub position : Position,
    pub velocity : Vec2D,
    pub state : IllnessState,
}

impl Person{
    pub fn new(world: World, position: Position, velocity: Vec2D, state: IllnessState) -> Self {
        Person { world, position, velocity, state }
    }

    pub fn step(&mut self){
        self.position = self.position.add(self.velocity);
        if self.position.x > self.world.width as f64{
            self.position.x = 0.0;
        }

        if self.position.x < 0.0{
            self.position.x = self.world.width as f64;
        }

        if self.position.y > self.world.height as f64{
            self.position.y = 0.0;
        }

        if self.position.y < 0.0{
            self.position.y = self.world.height as f64;
        }
    }

}