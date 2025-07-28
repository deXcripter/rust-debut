pub fn enums() {
    let direction = Direction::East;
    println!("This direction is: {}", direction.say_direction());

    let direction = Direction::West;
    println!("This direction is: {}", direction.say_direction());

    let direction = Direction::North;
    println!("This direction is: {}", direction.say_direction());

    let direction = Direction::South;
    println!("This direction is: {}", direction.say_direction());
}

pub enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    fn say_direction(&self) -> &str {
        match self {
            Direction::North => "Up",
            Direction::East => "Right",
            Direction::West => "Left",
            Direction::South => "Down",
        }
    }
}

pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn print_shape_area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => radius * 3.14 * radius,
            Shape::Rectangle(length, bredth) => length * bredth,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                return s;
            }
        }
    }
}

pub fn enums_with_data() {
    let shape = Shape::Circle(20.00);
    println!("The area of your circle is {}", shape.print_shape_area());

    let shape = Shape::Rectangle(20.00, 10.00);
    println!("The area of your rectangle is {}", shape.print_shape_area());

    let shape = Shape::Triangle(20.00, 10.00, 10.00);
    println!("The area of your triangle is {}", shape.print_shape_area())
}
