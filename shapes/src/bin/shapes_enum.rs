use shapes::{Circle, Rectangle, Square};

fn main() {
    pub enum Shape  {
        Circle(Circle),
        Square(Square),
        Rectangle(Rectangle),
    }


    impl Shape {
        pub fn area(&self) -> f64 {
            match self {
                Shape::Circle(c) => c.area(),
                Shape::Square(s) => s.area(),
                Shape::Rectangle(r) => r.area(),
            }
        }
    }



    let mut v: Vec<Shape> = Vec::new();
    v.push(Shape::Circle(Circle {radius: 3.14}));
    v.push(Shape::Square(Square{side_length: 2.0}));

}