use shapes::Circle;

fn main() {
    let c = Circle{radius: 3.14};
    println!("Circle with radius {} has area {}", c.radius, c.area());
}