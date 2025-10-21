

use shapes::shapes_enum::*;

fn main() {
    let c = Circle::new(3.14);
    println!("Circle with radius {} has area {}", c.radius, c.circle_area());
}

