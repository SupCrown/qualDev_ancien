

use shapes::shapes_enum::*;


#[test]
fn can_create_circle(){
    let c = Circle {radius: 3.14};
    assert_eq!(c.radius, 3.14);
}

#[test]
fn can_create_square(){
    let s = Square {side_lentgh: 3.14};
    assert_eq!(s.side_lentgh, 3.14);
}

#[test]
fn can_create_rectangle(){
    let r = Rectangle {width: 3.14, height: 3.14};
    assert_eq!(r.width, 3.14);
    assert_eq!(r.height, 3.14);
}

#[test]
fn can_create_shapes(){
    let mut v: Vec<Shape> = Vec::new();
    v.push(Shape::Circle(Circle::new(3.14)));
    v.push(Shape::Square(Square::new(2.0)));
    v.push(Shape::Rectangle(Rectangle::new(2.0,3.0)));
}

#[test]
fn can_use_function_shape_circle(){
    let shape = Shape::Circle(Circle::new(3.14));
    match shape {
        Shape::Circle(c) => {  // Test if the shape is a `Circle` variant
            assert_eq!(c.radius, 3.14);
        }
        _ => panic!("Expected Circle variant"),
    }
}

#[test]
fn can_use_function_shape_square(){
    let shape = Shape::Square(Square::new(3.14));
    match shape {
        Shape::Square(s) => {  // Test if the shape is a `Square` variant
            assert_eq!(s.side_lentgh, 3.14);
        }
        _ => panic!("Expected Square variant"),
    }
}


#[test]
fn can_use_function_shape_rectangle(){
    let shape = Shape::Rectangle(Rectangle::new(3.14, 3.14));
    match shape {
        Shape::Rectangle(r) => {  // Test if the shape is a `Rectangle` variant
            assert_eq!(r.width, 3.14);
            assert_eq!(r.height, 3.14);
        }
        _ => panic!("Expected Rectangle variant"),
    }
}



