pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub struct Circle {
    pub radius : f64,
}

pub struct Square {
    pub side_lentgh : f64,
}

pub struct Rectangle {
    pub width : f64,
    pub height: f64,
}

impl Circle {


    /// Returns circle's area
    /// ```
    /// # use shapes::Circle;
    /// let c = Circle {radius: 1.0};
    /// assert_eq!(c.circle_area(), std::f64::consts::PI * c.radius.powi(2));
    /// ```
    ///
    /// Returns circle's perimeter
    /// ```
    /// # use shapes::Circle;
    /// let c = Circle {radius: 1.0};
    /// assert_eq!(c.circle_perimeter(), 2.0 * std::f64::consts::PI * c.radius.powi(2));
    /// ```


    pub fn circle_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    pub fn circle_perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius.powi(2)
    }
}

use std::fmt;

impl fmt::Display for Circle {

    /// Returns a Circle in the console
    /// ```
    /// use shapes::Circle;
    /// let circle = Circle {radius: 3.14};
    ///let s = format!("{}", circle);
    ///assert_eq!(s, "A Circle with radius 3.14");
    /// ```

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A Circle with radius {}", self.radius)
    }
}

impl Square {

    /// Returns square's area
    /// ```
    /// # use shapes::Square;
    ///  let s = Square {side_lentgh: 3.14};
    /// assert_eq!(s.square_area(), s.side_lentgh * s.side_lentgh);
    /// ```
    ///
    /// Returns square's perimeter
    /// ```
    /// # use shapes::Square;
    ///  let s = Square {side_lentgh: 3.14};
    /// assert_eq!(s.square_perimeter(), 4.0  * s.side_lentgh);
    /// ```

    pub fn square_area(&self) -> f64 {
        self.side_lentgh * self.side_lentgh
    }

    pub fn square_perimeter(&self) -> f64 {
        4.0 * self.side_lentgh
    }
}

impl fmt::Display for Square {

    /// Returns a Square in the console
    /// ```
    /// use shapes::Square;
    /// let square = Square {side_lentgh: 3.14};
    ///  let s = format!("{}", square);
    ///  assert_eq!(s, "A Square with side_lenght 3.14");
    /// ```

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A Square with side_lenght {}", self.side_lentgh)
    }
}

impl Rectangle {

    /// Returns rectangle's area
    /// ```
    /// # use shapes::Rectangle;
    ///  let r = Rectangle {width: 3.14, height: 3.14};
    /// assert_eq!(r.rectangle_area(), r.width * r.height);
    /// ```
    ///
    /// Returns rectangle's perimeter
    /// ```
    /// # use shapes::Rectangle;
    ///  let r = Rectangle {width: 3.14, height: 3.14};
    /// assert_eq!(r.rectangle_perimeter(), r.width * r.height * 2.0);
    /// ```

    pub fn rectangle_area(&self) -> f64 {
        self.width * self.height
    }
    pub fn rectangle_perimeter(&self) -> f64 {
        self.width * self.height * 2.0
    }
}

impl fmt::Display for Rectangle {

    /// Returns a Rectangle in the console
    /// ```
    ///  use shapes::Rectangle;
    /// let rectangle = Rectangle {width: 3.14, height: 3.14};
    ///  let s = format!("{}", rectangle);
    ///  assert_eq!(s, "A Rectangle with width 3.14 and height 3.14");
    /// ```

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A Rectangle with width {} and height {}", self.width, self.height)
    }
}

impl Circle {

    /// Returns a new Circle scale
    /// ```
    ///  use shapes::Circle;
    /// let circle = Circle{radius: 3.14};
    ///  let facteur = 4.0;
    ///  assert_eq!(circle.scalecircle(facteur), circle.radius * facteur);
    /// ```

    pub fn scalecircle(&self, facteur: f64 ) -> f64 {
        facteur*self.radius
    }
}

impl Square {

    /// Returns a new Square scale
    /// ```
    /// use shapes::Square;
    /// let square = Square {side_lentgh: 3.14};
    ///  let facteur = 4.0;
    ///  assert_eq!(square.scalesquare(facteur), square.side_lentgh * facteur)
    /// ```

    pub fn scalesquare(&self, facteur: f64 ) -> f64 {
        facteur*self.side_lentgh
    }
}

impl Rectangle {

    /// Returns a new Rectangle scale
    /// ```
    ///  use shapes::Rectangle;
    /// let rectangle = Rectangle {width: 3.14, height: 3.14};
    ///   let facteur = 4.0;
    ///  assert_eq!(rectangle.scalerectangle(facteur), rectangle.width * rectangle.height * facteur);
    /// ```

    pub fn scalerectangle(&self, facteur: f64 ) -> f64 {
        facteur*self.width*self.height
    }
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Square {
    pub fn new(side_lentgh: f64) -> Square {
        Square { side_lentgh }
    }
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

pub enum Shape  {
    Circle(Circle),
    Square(Square),
    Rectangle(Rectangle),
}



impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.circle_area(),
            Shape::Square(s) => s.square_area(),
            Shape::Rectangle(r) => r.rectangle_area(),
        }
    }
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.circle_perimeter(),
            Shape::Square(s) => s.square_perimeter(),
            Shape::Rectangle(r) => r.rectangle_perimeter(),
        }
    }
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_circle(){
        let a_circle = Circle {radius: 3.14};
        assert_eq!(a_circle.radius, 3.14);
    }

    #[test]
    fn can_create_square(){
        let square = Square {side_lentgh: 3.14};
        assert_eq!(square.side_lentgh, 3.14);
    }

    #[test]
    fn can_create_rectangle(){
        let a_rectangle = Rectangle {width: 3.14, height: 3.14};
        assert_eq!(a_rectangle.width, 3.14);
    }

    #[test]
    fn can_calculate_area_and_perimeter_circle(){
        let circle = Circle {radius: 3.14};
        assert_eq!(circle.circle_area(), std::f64::consts::PI * circle.radius.powi(2));
        assert_eq!(circle.circle_perimeter(), 2.0 * std::f64::consts::PI * circle.radius.powi(2));
    }

    #[test]
    fn can_calculate_area_and_perimeter_square(){
        let square = Square {side_lentgh: 3.14};
        assert_eq!(square.square_area(), square.side_lentgh * square.side_lentgh);
        assert_eq!(square.square_perimeter(), 4.0  * square.side_lentgh);
    }

    #[test]
    fn can_calculate_area_and_perimeter_rectangle(){
        let rectangle = Rectangle {width: 3.14, height: 3.14};
        assert_eq!(rectangle.rectangle_area(), rectangle.width * rectangle.height);
        assert_eq!(rectangle.rectangle_perimeter(), rectangle.width * rectangle.height * 2.0);
    }

    #[test]
    fn can_use_circle_display(){
        let circle = Circle {radius: 3.14};
        let s = format!("{}", circle);
        assert_eq!(s, "A Circle with radius 3.14");
    }

    #[test]
    fn can_use_square_display(){
        let square = Square {side_lentgh: 3.14};
        let s = format!("{}", square);
        assert_eq!(s, "A Square with side_lenght 3.14");
    }

    #[test]
    fn can_use_rectangle_display(){
        let rectangle = Rectangle {width: 3.14, height: 3.14};
        let s = format!("{}", rectangle);
        assert_eq!(s, "A Rectangle with width 3.14 and height 3.14");
    }

    #[test]
    fn can_use_circle_scale(){
        let circle = Circle {radius: 3.14};
        let facteur = 4.0;
        assert_eq!(circle.scalecircle(facteur), circle.radius * facteur);
    }

    #[test]
    fn can_use_square_scale(){
        let square = Square {side_lentgh: 3.14};
        let facteur = 4.0;
        assert_eq!(square.scalesquare(facteur), square.side_lentgh * facteur)
    }

    #[test]
    fn can_use_rectangle_scale(){
        let rectangle = Rectangle {width: 3.14, height: 3.14};
        let facteur = 4.0;
        assert_eq!(rectangle.scalerectangle(facteur), rectangle.width * rectangle.height * facteur);
    }
}
