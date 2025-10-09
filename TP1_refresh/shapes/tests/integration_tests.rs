
use shapes::Circle;



#[test]
fn can_create_circle(){
    let c = Circle {radius: 3.14};
    assert_eq!(c.radius, 3.14);
}


