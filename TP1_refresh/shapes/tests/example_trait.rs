use shapes::Shapetrait;

fn main() {

    let circle = shapes::shapes_enum::Circle { radius: 2.0 };
    let square = shapes::shapes_enum::Square { side_lentgh: 3.0 };
    let rect = shapes::shapes_enum::Rectangle { width: 2.0, height: 5.0 };


    let shapes: Vec<Box<dyn Shapetrait>> = vec![
        Box::new(circle),
        Box::new(square),
        Box::new(rect),
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!(
            "Shape {} -> area: {:.2}, perimeter: {:.2}",
            i + 1,
            shape.area(),
            shape.perimeter()
        );
    }
}