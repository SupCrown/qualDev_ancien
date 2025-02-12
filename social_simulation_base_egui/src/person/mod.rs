use egui::scroll_area::State;
use crate::person::position::Position;
use crate::person::vec2d::Vec2D;
use crate::world::World;
use illness_state::IllnessState;

mod illness_state;
mod vec2d;
mod position;


pub struct Person{
    world: World,
    pub position : Position,
    pub velocity : Vec2D,
    pub state : IllnessState,
}

impl Person{
    pub fn new(world: World, position: Position, vec2d: Vec2D, state: IllnessState) -> Self {
        Person { world, position, velocity, state }
    }

    pub fn step(&mut self){
        self.position = self.position.add(self.velocity);
    }

}