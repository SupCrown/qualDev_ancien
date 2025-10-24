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

    pub struct RightTriangle {
        pub a: f64, // côté 1 (perpendiculaire)
        pub b: f64, // côté 2 (perpendiculaire)
    }

    impl Shapetrait for RightTriangle {
        fn area(&self) -> f64 {
            0.5 * self.a * self.b
        }
        fn perimeter(&self) -> f64 {
            let c = (self.a * self.a + self.b * self.b).sqrt();
            self.a + self.b + c
        }
    }

    fn main() {

        let c = shapes::shapes_enum::Circle { radius: 2.0 };
        let s = shapes::shapes_enum::Square { side_lentgh: 3.0 };
        let r = shapes::shapes_enum::Rectangle { width: 2.0, height: 5.0 };

        let t = RightTriangle { a: 3.0, b: 4.0 };

        let shapes_boxed: Vec<Box<dyn Shapetrait>> = vec![
            Box::new(c),
            Box::new(s),
            Box::new(r),
            Box::new(t),
        ];

        for (i, sh) in shapes_boxed.iter().enumerate() {
            println!(
                "Shape {} -> area: {:.2}, perimeter: {:.2}",
                i + 1,
                sh.area(),
                sh.perimeter()
            );
        }

        let refs: Vec<&dyn Shapetrait> = shapes_boxed.iter().map(|b| b.as_ref()).collect();
        let sum = shapes::total_area(&refs);
        println!("Total area = {:.2}", sum);
    }


}